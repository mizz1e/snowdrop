use {
    clap::builder::{self, EnumValueParser, PossibleValue, TypedValueParser},
    std::{ffi, fmt, str},
};

#[derive(Clone, Debug)]
pub struct EnumFromStrValueParser<E>
where
    E: clap::ValueEnum + str::FromStr + Clone + Send + Sync + 'static,
    <E as str::FromStr>::Err: fmt::Display,
{
    inner: EnumValueParser<E>,
}

impl<E> EnumFromStrValueParser<E>
where
    E: clap::ValueEnum + str::FromStr + Clone + Send + Sync + 'static,
    <E as str::FromStr>::Err: fmt::Display,
{
    /// Parse an [`ValueEnum`][clap::ValueEnum] using [`FromStr`](str::FromStr).
    pub fn new() -> Self {
        Self {
            inner: EnumValueParser::new(),
        }
    }
}

impl<E> Default for EnumFromStrValueParser<E>
where
    E: clap::ValueEnum + str::FromStr + Clone + Send + Sync + 'static,
    <E as str::FromStr>::Err: fmt::Display,
{
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<E> TypedValueParser for EnumFromStrValueParser<E>
where
    E: clap::ValueEnum + str::FromStr + Clone + Send + Sync + 'static,
    <E as str::FromStr>::Err: fmt::Display,
{
    type Value = E;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let string = builder::StringValueParser::new().parse_ref(cmd, arg, value)?;

        match string.parse::<E>() {
            Ok(value) => Ok(value),
            Err(_error) => self.inner.parse_ref(cmd, arg, value),
        }
    }

    #[inline]
    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        self.inner.possible_values()
    }
}
