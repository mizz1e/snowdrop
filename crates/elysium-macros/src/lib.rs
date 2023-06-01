#![deny(warnings)]

use {proc_macro::TokenStream, quote::ToTokens, std::mem, syn::spanned::Spanned};

/// Convenience macro to construct a vector of attributes.
macro_rules! attributes {
    ($(#[$meta:meta])*) => {{
        let attributes: Vec<::syn::Attribute> = vec![$(::syn::parse_quote!(#[$meta]),)*];

        attributes
    }}
}

/// Generate a console.
///
/// It is responsible for both command, and variable generation.
#[proc_macro_attribute]
pub fn console(_attribute: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::ItemEnum);

    try_console(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Main bulk of generating a console.
fn try_console(mut input: syn::ItemEnum) -> syn::Result<proc_macro2::TokenStream> {
    input.attrs = mem::take(&mut input.attrs)
        .into_iter()
        .flat_map(|attribute| {
            // Prevents manual use of `#[command]`.
            if attribute.path().is_ident("command") {
                Some(Err(syn::Error::new(
                    attribute.span(),
                    "\"command\" is used internally",
                )))
            } else {
                Some(Ok(attribute))
            }
        })
        .collect::<syn::Result<_>>()?;

    input.attrs.extend(attributes!(
        #[derive(::clap::Parser)]
        // No arguments will display help.
        #[command(arg_required_else_help = true)]
        // Always called "console".
        #[command(bin_name = "console")]
        // Don't provide the `help` command.
        #[command(disable_help_subcommand = true)]
        // Allows parsing `["sv_cheats"]`, rather than `["console", "sv_cheats"]`.
        #[command(multicall = true)]
        // Always called "console".
        #[command(name = "console")]
        // Automatically covers most cases.
        #[command(rename_all = "snake_case")]
        // Include them full stops, lol.
        #[command(verbatim_doc_comment)]
    ));

    for variant in input.variants.iter_mut() {
        // Validate the variant type.
        if !matches!(variant.fields, syn::Fields::Named(_) | syn::Fields::Unit) {
            return Err(syn::Error::new(
                variant.span(),
                "variants must be unit or have named fields",
            ));
        }

        let mut custom_name = None;

        variant.attrs = mem::take(&mut variant.attrs)
            .into_iter()
            .flat_map(|attribute| {
                if attribute.path().is_ident("console") {
                    let result = attribute.parse_nested_meta(|meta| {
                        // Parses `name = "custom_name"`.
                        if meta.path.is_ident("name") {
                            let value = meta.value()?;
                            let name: syn::LitStr = value.parse()?;

                            custom_name = Some(name);

                            Ok(())
                        } else {
                            Err(meta.error("unsupported argument to \"console\""))
                        }
                    });

                    match result {
                        Ok(()) => None,
                        Err(error) => Some(Err(error)),
                    }
                } else if attribute.path().is_ident("command") {
                    // Prevents manual use of `#[command]`.
                    Some(Err(syn::Error::new(
                        attribute.span(),
                        "\"command\" is used internally",
                    )))
                } else {
                    Some(Ok(attribute))
                }
            })
            .collect::<syn::Result<_>>()?;

        // Variants with fields take arguments, if theres no
        // arguments, show help.
        if !variant.fields.is_empty() {
            variant.attrs.extend(attributes!(
                // Display help by default for commands which require arguments.
                #[command(arg_required_else_help = true)]
            ));
        }

        // Rename if requested.
        if let Some(custom_name) = custom_name {
            variant.attrs.push(syn::parse_quote!(
                // `SvAutoBunnyHopping` is turned into `sv_auto_bunny_hopping` by default.
                // Custom overrides are needed.
                #[command(name = #custom_name)]
            ));
        }

        // Always use doc comments, verbatim.
        variant.attrs.extend(attributes!(
            // Include them full stops, lol.
            #[command(verbatim_doc_comment)]
        ));

        for field in variant.fields.iter_mut() {
            let mut default = None;

            field.attrs = mem::take(&mut field.attrs)
                .into_iter()
                .flat_map(|attribute| {
                    if attribute.path().is_ident("arg") {
                        let result = attribute.parse_nested_meta(|meta| {
                            // Parses `default = ???`.
                            if meta.path.is_ident("default") {
                                let value = meta.value()?;
                                let expr: syn::Expr = value.parse()?;

                                default = Some(expr);

                                Ok(())
                            } else {
                                Err(meta.error("unsupported argument to \"arg\""))
                            }
                        });

                        match result {
                            Ok(()) => None,
                            Err(error) => Some(Err(error)),
                        }
                    } else {
                        Some(Ok(attribute))
                    }
                })
                .collect::<syn::Result<_>>()?;

            // Rename if requested.
            if let Some(default) = default {
                field.attrs.push(syn::parse_quote!(
                    #[arg(default_value_t = #default)]
                ));
            }

            field.attrs.extend(attributes!(
                // Prevent clap being confused.
                // See https://github.com/clap-rs/clap/issues/4467
                #[arg(action = ArgAction::Set)]

                // Allows a default value to be displayed while being required still.
                #[arg(required = true)]
            ));
        }
    }

    Ok(input.to_token_stream())
}
