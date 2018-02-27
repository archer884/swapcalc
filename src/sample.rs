use chrono::{DateTime, Utc};
use error::ParseError;
use std::error;
use std::str::FromStr;

struct ParseContext<'a> {
    context: Option<&'a str>,
}

impl<'a> ParseContext<'a> {
    fn new(context: Option<&'a str>) -> Self {
        Self { context }
    }

    fn parse<T, E>(&self) -> Result<T, ParseError>
    where
        T: FromStr<Err = E>,
        E: error::Error + 'static,
    {
        let context = self.context.ok_or(ParseError::MissingColumn)?;
        context.parse().map_err(|e| ParseError::failure(e, context))
    }
}

pub struct Sample {
    pub timestamp: DateTime<Utc>,
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_free: u64,
}

impl FromStr for Sample {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut columns = s.split(',');

        Ok(Self {
            timestamp: ParseContext::new(columns.next()).parse()?,
            total: ParseContext::new(columns.next()).parse()?,
            free: ParseContext::new(columns.next()).parse()?,
            available: ParseContext::new(columns.next()).parse()?,
            buffers: ParseContext::new(columns.next()).parse()?,
            cached:  ParseContext::new(columns.next()).parse()?,
            swap_total: ParseContext::new(columns.next()).parse()?,
            swap_free: ParseContext::new(columns.next()).parse()?,
        })
    }
}
