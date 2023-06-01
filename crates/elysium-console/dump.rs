#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use clap::{ArgAction, Parser, ValueEnum};
#[repr(u8)]
pub enum Impacts {
    /// Don't show any.
    #[default]
    Hidden,
    /// Show client (red), and server (blue).
    Both,
    /// Show client (red).
    Client,
    /// Show server (blue).
    Server,
}
#[automatically_derived]
impl ::core::clone::Clone for Impacts {
    #[inline]
    fn clone(&self) -> Impacts {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Impacts {}
#[automatically_derived]
impl ::core::fmt::Debug for Impacts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Impacts::Hidden => "Hidden",
                Impacts::Both => "Both",
                Impacts::Client => "Client",
                Impacts::Server => "Server",
            },
        )
    }
}
#[automatically_derived]
impl ::core::default::Default for Impacts {
    #[inline]
    fn default() -> Impacts {
        Self::Hidden
    }
}
#[automatically_derived]
impl ::core::marker::StructuralEq for Impacts {}
#[automatically_derived]
impl ::core::cmp::Eq for Impacts {
    #[inline]
    #[doc(hidden)]
    #[no_coverage]
    fn assert_receiver_is_total_eq(&self) -> () {}
}
#[automatically_derived]
impl ::core::hash::Hash for Impacts {
    #[inline]
    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        ::core::hash::Hash::hash(&__self_tag, state)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Impacts {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Impacts {
    #[inline]
    fn eq(&self, other: &Impacts) -> bool {
        let __self_tag = ::core::intrinsics::discriminant_value(self);
        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
        __self_tag == __arg1_tag
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
impl clap::ValueEnum for Impacts {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Hidden, Self::Both, Self::Client, Self::Server]
    }
    fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue> {
        match self {
            Self::Hidden => {
                Some({ clap::builder::PossibleValue::new("hidden").help("Don't show any") })
            }
            Self::Both => Some({
                clap::builder::PossibleValue::new("both")
                    .help("Show client (red), and server (blue)")
            }),
            Self::Client => {
                Some({ clap::builder::PossibleValue::new("client").help("Show client (red)") })
            }
            Self::Server => {
                Some({ clap::builder::PossibleValue::new("server").help("Show server (blue)") })
            }
            _ => None,
        }
    }
}
#[command(arg_required_else_help = true)]
#[command(bin_name = "console")]
#[command(disable_help_subcommand = true)]
#[command(multicall = true)]
#[command(name = "console")]
#[command(rename_all = "snake_case")]
#[command(verbatim_doc_comment)]
pub enum Console {
    /// Toggle buddah mode.
    ///
    /// Forces you to have 1 health.
    ///
    /// Requires sv_cheats.
    #[command(verbatim_doc_comment)]
    Buddah,
    /// Print the position of the player.
    #[command(name = "getpos")]
    #[command(verbatim_doc_comment)]
    GetPos,
    /// Toggle invincibility.
    ///
    /// Requires sv_cheats.
    #[command(verbatim_doc_comment)]
    God,
    /// Restart the game in n seconds.
    ///
    /// Requires sv_cheats.
    #[command(arg_required_else_help = true)]
    #[command(name = "mp_restartgame")]
    #[command(verbatim_doc_comment)]
    MpRestartGame {
        #[arg(default_value_t = 0.0)]
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        seconds: f32,
    },
    /// Toggle collision.
    ///
    /// Requires sv_cheats.
    #[command(name = "noclip")]
    #[command(verbatim_doc_comment)]
    NoClip,
    /// Toggle NPC detection of the player.
    ///
    /// Requires sv_cheats.
    #[command(name = "notarget")]
    #[command(verbatim_doc_comment)]
    NoTarget,
    /// Set the coordinates of the player.
    #[command(arg_required_else_help = true)]
    #[command(name = "setpos")]
    #[command(verbatim_doc_comment)]
    SetPos {
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        x: f32,
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        y: f32,
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        z: f32,
    },
    /// Air acceleration modifier.
    #[command(arg_required_else_help = true)]
    #[command(name = "sv_airaccelerate")]
    #[command(verbatim_doc_comment)]
    SvAirAccelerate {
        #[arg(default_value_t = 0.0)]
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        accelerate: f32,
    },
    /// Automatically bunny hop.
    #[command(arg_required_else_help = true)]
    #[command(name = "sv_autobunnyhopping")]
    #[command(verbatim_doc_comment)]
    SvAutoBunnyHopping {
        #[arg(default_value_t = false)]
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        bunny_hopping: bool,
    },
    /// Whether cheats are enabled on the server.
    #[command(arg_required_else_help = true)]
    #[command(verbatim_doc_comment)]
    SvCheats {
        #[arg(default_value_t = false)]
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        cheats: bool,
    },
    /// Whether bunny hopping is allowed.
    #[command(arg_required_else_help = true)]
    #[command(name = "sv_enablebunnyhopping")]
    #[command(verbatim_doc_comment)]
    SvEnableBunnyHopping {
        #[arg(default_value_t = false)]
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        bunny_hopping: bool,
    },
    /// Gravity applied to all entities.
    #[command(arg_required_else_help = true)]
    #[command(verbatim_doc_comment)]
    SvGravity {
        #[arg(default_value_t = 800.0)]
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        gravity: f32,
    },
    /// Whether to verify content with the server.
    #[command(arg_required_else_help = true)]
    #[command(verbatim_doc_comment)]
    SvPure {
        #[arg(default_value_t = true)]
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        pure: bool,
    },
    /// Show bullet impacts.
    #[command(arg_required_else_help = true)]
    #[command(name = "sv_showimpacts")]
    #[command(verbatim_doc_comment)]
    SvShowImpacts {
        #[arg(action = ArgAction::Set)]
        #[arg(required = true)]
        impacts: Impacts,
    },
}
impl clap::Parser for Console {}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
impl clap::CommandFactory for Console {
    fn command<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("console")
            .subcommand_required(true)
            .arg_required_else_help(true);
        <Self as clap::Subcommand>::augment_subcommands(__clap_app)
    }
    fn command_for_update<'b>() -> clap::Command {
        let __clap_app = clap::Command::new("console");
        <Self as clap::Subcommand>::augment_subcommands_for_update(__clap_app)
            .subcommand_required(false)
            .arg_required_else_help(false)
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
impl clap::FromArgMatches for Console {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn from_arg_matches_mut(
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        #![allow(deprecated)]
        if let Some((__clap_name, mut __clap_arg_sub_matches)) =
            __clap_arg_matches.remove_subcommand()
        {
            let __clap_arg_matches = &mut __clap_arg_sub_matches;
            if __clap_name == "buddah" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::Buddah);
            }
            if __clap_name == "getpos" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::GetPos);
            }
            if __clap_name == "god" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::God);
            }
            if __clap_name == "mp_restartgame" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::MpRestartGame {
                    seconds: __clap_arg_matches
                        .remove_one::<f32>("seconds")
                        .ok_or_else(|| {
                            clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                let res = ::alloc::fmt::format(format_args!(
                                    "The following required argument was not provided: {0}",
                                    "seconds"
                                ));
                                res
                            })
                        })?,
                });
            }
            if __clap_name == "noclip" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::NoClip);
            }
            if __clap_name == "notarget" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::NoTarget);
            }
            if __clap_name == "setpos" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::SetPos {
                    x: __clap_arg_matches.remove_one::<f32>("x").ok_or_else(|| {
                        clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                            let res = ::alloc::fmt::format(format_args!(
                                "The following required argument was not provided: {0}",
                                "x"
                            ));
                            res
                        })
                    })?,
                    y: __clap_arg_matches.remove_one::<f32>("y").ok_or_else(|| {
                        clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                            let res = ::alloc::fmt::format(format_args!(
                                "The following required argument was not provided: {0}",
                                "y"
                            ));
                            res
                        })
                    })?,
                    z: __clap_arg_matches.remove_one::<f32>("z").ok_or_else(|| {
                        clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                            let res = ::alloc::fmt::format(format_args!(
                                "The following required argument was not provided: {0}",
                                "z"
                            ));
                            res
                        })
                    })?,
                });
            }
            if __clap_name == "sv_airaccelerate" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::SvAirAccelerate {
                    accelerate: __clap_arg_matches
                        .remove_one::<f32>("accelerate")
                        .ok_or_else(|| {
                            clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                let res = ::alloc::fmt::format(format_args!(
                                    "The following required argument was not provided: {0}",
                                    "accelerate"
                                ));
                                res
                            })
                        })?,
                });
            }
            if __clap_name == "sv_autobunnyhopping" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::SvAutoBunnyHopping {
                    bunny_hopping: __clap_arg_matches
                        .remove_one::<bool>("bunny_hopping")
                        .ok_or_else(|| {
                            clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                let res = ::alloc::fmt::format(format_args!(
                                    "The following required argument was not provided: {0}",
                                    "bunny_hopping"
                                ));
                                res
                            })
                        })?,
                });
            }
            if __clap_name == "sv_cheats" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::SvCheats {
                    cheats: __clap_arg_matches
                        .remove_one::<bool>("cheats")
                        .ok_or_else(|| {
                            clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                let res = ::alloc::fmt::format(format_args!(
                                    "The following required argument was not provided: {0}",
                                    "cheats"
                                ));
                                res
                            })
                        })?,
                });
            }
            if __clap_name == "sv_enablebunnyhopping" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::SvEnableBunnyHopping {
                    bunny_hopping: __clap_arg_matches
                        .remove_one::<bool>("bunny_hopping")
                        .ok_or_else(|| {
                            clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                let res = ::alloc::fmt::format(format_args!(
                                    "The following required argument was not provided: {0}",
                                    "bunny_hopping"
                                ));
                                res
                            })
                        })?,
                });
            }
            if __clap_name == "sv_gravity" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::SvGravity {
                    gravity: __clap_arg_matches
                        .remove_one::<f32>("gravity")
                        .ok_or_else(|| {
                            clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                let res = ::alloc::fmt::format(format_args!(
                                    "The following required argument was not provided: {0}",
                                    "gravity"
                                ));
                                res
                            })
                        })?,
                });
            }
            if __clap_name == "sv_pure" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::SvPure {
                    pure: __clap_arg_matches
                        .remove_one::<bool>("pure")
                        .ok_or_else(|| {
                            clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                let res = ::alloc::fmt::format(format_args!(
                                    "The following required argument was not provided: {0}",
                                    "pure"
                                ));
                                res
                            })
                        })?,
                });
            }
            if __clap_name == "sv_showimpacts" && !__clap_arg_matches.contains_id("") {
                return ::std::result::Result::Ok(Self::SvShowImpacts {
                    impacts: __clap_arg_matches
                        .remove_one::<Impacts>("impacts")
                        .ok_or_else(|| {
                            clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                let res = ::alloc::fmt::format(format_args!(
                                    "The following required argument was not provided: {0}",
                                    "impacts"
                                ));
                                res
                            })
                        })?,
                });
            }
            ::std::result::Result::Err(clap::Error::raw(
                clap::error::ErrorKind::InvalidSubcommand,
                {
                    let res = ::alloc::fmt::format(format_args!(
                        "The subcommand \'{0}\' wasn\'t recognized",
                        __clap_name
                    ));
                    res
                },
            ))
        } else {
            ::std::result::Result::Err(clap::Error::raw(
                clap::error::ErrorKind::MissingSubcommand,
                "A subcommand is required but one was not provided.",
            ))
        }
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
    }
    fn update_from_arg_matches_mut<'b>(
        &mut self,
        __clap_arg_matches: &mut clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        #![allow(deprecated)]
        if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
            match self {
                Self::Buddah if "buddah" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::GetPos if "getpos" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::God if "god" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::MpRestartGame { seconds } if "mp_restartgame" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("seconds") {
                            *seconds = __clap_arg_matches
                                .remove_one::<f32>("seconds")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "The following required argument was not provided: {0}",
                                                "seconds"
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                Self::NoClip if "noclip" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::NoTarget if "notarget" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {}
                }
                Self::SetPos { x, y, z } if "setpos" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("x") {
                            *x = __clap_arg_matches.remove_one::<f32>("x").ok_or_else(|| {
                                clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                    let res = ::alloc::fmt::format(format_args!(
                                        "The following required argument was not provided: {0}",
                                        "x"
                                    ));
                                    res
                                })
                            })?;
                        }
                        if __clap_arg_matches.contains_id("y") {
                            *y = __clap_arg_matches.remove_one::<f32>("y").ok_or_else(|| {
                                clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                    let res = ::alloc::fmt::format(format_args!(
                                        "The following required argument was not provided: {0}",
                                        "y"
                                    ));
                                    res
                                })
                            })?;
                        }
                        if __clap_arg_matches.contains_id("z") {
                            *z = __clap_arg_matches.remove_one::<f32>("z").ok_or_else(|| {
                                clap::Error::raw(clap::error::ErrorKind::MissingRequiredArgument, {
                                    let res = ::alloc::fmt::format(format_args!(
                                        "The following required argument was not provided: {0}",
                                        "z"
                                    ));
                                    res
                                })
                            })?;
                        }
                    }
                }
                Self::SvAirAccelerate { accelerate } if "sv_airaccelerate" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("accelerate") {
                            *accelerate = __clap_arg_matches
                                .remove_one::<f32>("accelerate")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "The following required argument was not provided: {0}",
                                                "accelerate"
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                Self::SvAutoBunnyHopping { bunny_hopping }
                    if "sv_autobunnyhopping" == __clap_name =>
                {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("bunny_hopping") {
                            *bunny_hopping = __clap_arg_matches
                                .remove_one::<bool>("bunny_hopping")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "The following required argument was not provided: {0}",
                                                "bunny_hopping"
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                Self::SvCheats { cheats } if "sv_cheats" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("cheats") {
                            *cheats = __clap_arg_matches
                                .remove_one::<bool>("cheats")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "The following required argument was not provided: {0}",
                                                "cheats"
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                Self::SvEnableBunnyHopping { bunny_hopping }
                    if "sv_enablebunnyhopping" == __clap_name =>
                {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("bunny_hopping") {
                            *bunny_hopping = __clap_arg_matches
                                .remove_one::<bool>("bunny_hopping")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "The following required argument was not provided: {0}",
                                                "bunny_hopping"
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                Self::SvGravity { gravity } if "sv_gravity" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("gravity") {
                            *gravity = __clap_arg_matches
                                .remove_one::<f32>("gravity")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "The following required argument was not provided: {0}",
                                                "gravity"
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                Self::SvPure { pure } if "sv_pure" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("pure") {
                            *pure = __clap_arg_matches
                                .remove_one::<bool>("pure")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "The following required argument was not provided: {0}",
                                                "pure"
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                Self::SvShowImpacts { impacts } if "sv_showimpacts" == __clap_name => {
                    let (_, mut __clap_arg_sub_matches) =
                        __clap_arg_matches.remove_subcommand().unwrap();
                    let __clap_arg_matches = &mut __clap_arg_sub_matches;
                    {
                        if __clap_arg_matches.contains_id("impacts") {
                            *impacts = __clap_arg_matches
                                .remove_one::<Impacts>("impacts")
                                .ok_or_else(|| clap::Error::raw(
                                    clap::error::ErrorKind::MissingRequiredArgument,
                                    {
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "The following required argument was not provided: {0}",
                                                "impacts"
                                            ),
                                        );
                                        res
                                    },
                                ))?;
                        }
                    }
                }
                s => {
                    *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(__clap_arg_matches)?;
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting,
    clippy::almost_swapped
)]
impl clap::Subcommand for Console {
    fn augment_subcommands<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("buddah");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand.about("Toggle buddah mode.").long_about(
                "Toggle buddah mode.\n\nForces you to have 1 health.\n\nRequires sv_cheats.",
            )
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("getpos");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
                .about("Print the position of the player.")
                .long_about(None)
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("god");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
                .about("Toggle invincibility.")
                .long_about("Toggle invincibility.\n\nRequires sv_cheats.")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("mp_restartgame");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("MpRestartGame").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("seconds")];
                        members
                    }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("seconds")
                        .value_name("SECONDS")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: f32 = 0.0;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Restart the game in n seconds.")
                    .long_about("Restart the game in n seconds.\n\nRequires sv_cheats.")
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("noclip");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
                .about("Toggle collision.")
                .long_about("Toggle collision.\n\nRequires sv_cheats.")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("notarget");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
                .about("Toggle NPC detection of the player.")
                .long_about("Toggle NPC detection of the player.\n\nRequires sv_cheats.")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("setpos");
            {
                let __clap_subcommand =
                    __clap_subcommand.group(clap::ArgGroup::new("SetPos").multiple(true).args({
                        let members: [clap::Id; 3usize] = [
                            clap::Id::from("x"),
                            clap::Id::from("y"),
                            clap::Id::from("z"),
                        ];
                        members
                    }));
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("x")
                        .value_name("X")
                        .required(true && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg.required(true);
                    let arg = arg;
                    arg
                });
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("y")
                        .value_name("Y")
                        .required(true && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg.required(true);
                    let arg = arg;
                    arg
                });
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("z")
                        .value_name("Z")
                        .required(true && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg.required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Set the coordinates of the player.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_airaccelerate");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvAirAccelerate").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("accelerate")];
                        members
                    }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("accelerate")
                        .value_name("ACCELERATE")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: f32 = 0.0;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Air acceleration modifier.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_autobunnyhopping");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvAutoBunnyHopping")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("bunny_hopping")];
                            members
                        }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("bunny_hopping")
                        .value_name("BUNNY_HOPPING")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<bool>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: bool = false;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Automatically bunny hop.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_cheats");
            {
                let __clap_subcommand =
                    __clap_subcommand.group(clap::ArgGroup::new("SvCheats").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("cheats")];
                        members
                    }));
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("cheats")
                        .value_name("CHEATS")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<bool>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: bool = false;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Whether cheats are enabled on the server.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_enablebunnyhopping");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvEnableBunnyHopping")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("bunny_hopping")];
                            members
                        }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("bunny_hopping")
                        .value_name("BUNNY_HOPPING")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<bool>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: bool = false;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Whether bunny hopping is allowed.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_gravity");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvGravity").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("gravity")];
                        members
                    }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("gravity")
                        .value_name("GRAVITY")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: f32 = 800.0;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Gravity applied to all entities.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_pure");
            {
                let __clap_subcommand =
                    __clap_subcommand.group(clap::ArgGroup::new("SvPure").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("pure")];
                        members
                    }));
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("pure")
                        .value_name("PURE")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<bool>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: bool = true;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Whether to verify content with the server.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_showimpacts");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvShowImpacts").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("impacts")];
                        members
                    }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("impacts")
                        .value_name("IMPACTS")
                        .required(true && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<Impacts>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg.required(true);
                    let arg = arg;
                    arg
                });
                __clap_subcommand
                    .about("Show bullet impacts.    ")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        __clap_app
            .arg_required_else_help(true)
            .bin_name("console")
            .disable_help_subcommand(true)
            .multicall(true)
    }
    fn augment_subcommands_for_update<'b>(__clap_app: clap::Command) -> clap::Command {
        let __clap_app = __clap_app;
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("buddah");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand.about("Toggle buddah mode.").long_about(
                "Toggle buddah mode.\n\nForces you to have 1 health.\n\nRequires sv_cheats.",
            )
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("getpos");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
                .about("Print the position of the player.")
                .long_about(None)
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("god");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
                .about("Toggle invincibility.")
                .long_about("Toggle invincibility.\n\nRequires sv_cheats.")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("mp_restartgame");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("MpRestartGame").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("seconds")];
                        members
                    }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("seconds")
                        .value_name("SECONDS")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: f32 = 0.0;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Restart the game in n seconds.")
                    .long_about("Restart the game in n seconds.\n\nRequires sv_cheats.")
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("noclip");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
                .about("Toggle collision.")
                .long_about("Toggle collision.\n\nRequires sv_cheats.")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("notarget");
            let __clap_subcommand = __clap_subcommand;
            let __clap_subcommand = __clap_subcommand;
            __clap_subcommand
                .about("Toggle NPC detection of the player.")
                .long_about("Toggle NPC detection of the player.\n\nRequires sv_cheats.")
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("setpos");
            {
                let __clap_subcommand =
                    __clap_subcommand.group(clap::ArgGroup::new("SetPos").multiple(true).args({
                        let members: [clap::Id; 3usize] = [
                            clap::Id::from("x"),
                            clap::Id::from("y"),
                            clap::Id::from("z"),
                        ];
                        members
                    }));
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("x")
                        .value_name("X")
                        .required(true && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg.required(true);
                    let arg = arg.required(false);
                    arg
                });
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("y")
                        .value_name("Y")
                        .required(true && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg.required(true);
                    let arg = arg.required(false);
                    arg
                });
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("z")
                        .value_name("Z")
                        .required(true && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg.required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Set the coordinates of the player.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_airaccelerate");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvAirAccelerate").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("accelerate")];
                        members
                    }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("accelerate")
                        .value_name("ACCELERATE")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: f32 = 0.0;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Air acceleration modifier.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_autobunnyhopping");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvAutoBunnyHopping")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("bunny_hopping")];
                            members
                        }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("bunny_hopping")
                        .value_name("BUNNY_HOPPING")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<bool>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: bool = false;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Automatically bunny hop.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_cheats");
            {
                let __clap_subcommand =
                    __clap_subcommand.group(clap::ArgGroup::new("SvCheats").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("cheats")];
                        members
                    }));
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("cheats")
                        .value_name("CHEATS")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<bool>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: bool = false;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Whether cheats are enabled on the server.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_enablebunnyhopping");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvEnableBunnyHopping")
                        .multiple(true)
                        .args({
                            let members: [clap::Id; 1usize] = [clap::Id::from("bunny_hopping")];
                            members
                        }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("bunny_hopping")
                        .value_name("BUNNY_HOPPING")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<bool>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: bool = false;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Whether bunny hopping is allowed.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_gravity");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvGravity").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("gravity")];
                        members
                    }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("gravity")
                        .value_name("GRAVITY")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<f32>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: f32 = 800.0;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Gravity applied to all entities.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_pure");
            {
                let __clap_subcommand =
                    __clap_subcommand.group(clap::ArgGroup::new("SvPure").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("pure")];
                        members
                    }));
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("pure")
                        .value_name("PURE")
                        .required(false && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<bool>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg
                        .default_value({
                            static DEFAULT_VALUE: clap::__derive_refs::once_cell::sync::Lazy<
                                String,
                            > = clap::__derive_refs::once_cell::sync::Lazy::new(|| {
                                let val: bool = true;
                                ::std::string::ToString::to_string(&val)
                            });
                            let s: &'static str = &*DEFAULT_VALUE;
                            s
                        })
                        .required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Whether to verify content with the server.")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        let __clap_app = __clap_app.subcommand({
            let __clap_subcommand = clap::Command::new("sv_showimpacts");
            {
                let __clap_subcommand = __clap_subcommand.group(
                    clap::ArgGroup::new("SvShowImpacts").multiple(true).args({
                        let members: [clap::Id; 1usize] = [clap::Id::from("impacts")];
                        members
                    }),
                );
                let __clap_subcommand = __clap_subcommand.arg({
                    #[allow(deprecated)]
                    let arg = clap::Arg::new("impacts")
                        .value_name("IMPACTS")
                        .required(true && ArgAction::Set.takes_values())
                        .value_parser({
                            use ::clap_builder::builder::via_prelude::*;
                            let auto = ::clap_builder::builder::_AutoValueParser::<Impacts>::new();
                            (&&&&&&auto).value_parser()
                        })
                        .action(ArgAction::Set);
                    let arg = arg.required(true);
                    let arg = arg.required(false);
                    arg
                });
                __clap_subcommand
                    .about("Show bullet impacts.    ")
                    .long_about(None)
                    .arg_required_else_help(true)
            }
        });
        __clap_app
            .arg_required_else_help(true)
            .bin_name("console")
            .disable_help_subcommand(true)
            .multicall(true)
    }
    fn has_subcommand(__clap_name: &str) -> bool {
        if "buddah" == __clap_name {
            return true;
        }
        if "getpos" == __clap_name {
            return true;
        }
        if "god" == __clap_name {
            return true;
        }
        if "mp_restartgame" == __clap_name {
            return true;
        }
        if "noclip" == __clap_name {
            return true;
        }
        if "notarget" == __clap_name {
            return true;
        }
        if "setpos" == __clap_name {
            return true;
        }
        if "sv_airaccelerate" == __clap_name {
            return true;
        }
        if "sv_autobunnyhopping" == __clap_name {
            return true;
        }
        if "sv_cheats" == __clap_name {
            return true;
        }
        if "sv_enablebunnyhopping" == __clap_name {
            return true;
        }
        if "sv_gravity" == __clap_name {
            return true;
        }
        if "sv_pure" == __clap_name {
            return true;
        }
        if "sv_showimpacts" == __clap_name {
            return true;
        }
        false
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Console {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            Console::Buddah => ::core::fmt::Formatter::write_str(f, "Buddah"),
            Console::GetPos => ::core::fmt::Formatter::write_str(f, "GetPos"),
            Console::God => ::core::fmt::Formatter::write_str(f, "God"),
            Console::MpRestartGame { seconds: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "MpRestartGame",
                    "seconds",
                    &__self_0,
                )
            }
            Console::NoClip => ::core::fmt::Formatter::write_str(f, "NoClip"),
            Console::NoTarget => ::core::fmt::Formatter::write_str(f, "NoTarget"),
            Console::SetPos {
                x: __self_0,
                y: __self_1,
                z: __self_2,
            } => ::core::fmt::Formatter::debug_struct_field3_finish(
                f, "SetPos", "x", __self_0, "y", __self_1, "z", &__self_2,
            ),
            Console::SvAirAccelerate {
                accelerate: __self_0,
            } => ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "SvAirAccelerate",
                "accelerate",
                &__self_0,
            ),
            Console::SvAutoBunnyHopping {
                bunny_hopping: __self_0,
            } => ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "SvAutoBunnyHopping",
                "bunny_hopping",
                &__self_0,
            ),
            Console::SvCheats { cheats: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f, "SvCheats", "cheats", &__self_0,
                )
            }
            Console::SvEnableBunnyHopping {
                bunny_hopping: __self_0,
            } => ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "SvEnableBunnyHopping",
                "bunny_hopping",
                &__self_0,
            ),
            Console::SvGravity { gravity: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SvGravity",
                    "gravity",
                    &__self_0,
                )
            }
            Console::SvPure { pure: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(f, "SvPure", "pure", &__self_0)
            }
            Console::SvShowImpacts { impacts: __self_0 } => {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "SvShowImpacts",
                    "impacts",
                    &__self_0,
                )
            }
        }
    }
}
fn main() {
    let command = Console::parse_from(std::env::args_os().skip(1));
    {
        ::std::io::_print(format_args!("{0:?}\n", command));
    };
}
