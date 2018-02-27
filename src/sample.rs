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

    fn parse_explicit<T, F>(&self, parser: F) -> Result<T, ParseError>
    where
        F: FnOnce(&str) -> Result<T, ParseError>
    {
        let context = self.context.ok_or(ParseError::MissingColumn)?;
        parser(context)
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
            timestamp: ParseContext::new(columns.next()).parse_explicit(parse_timestamp)?,
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

fn parse_timestamp(s: &str) -> Result<DateTime<Utc>, ParseError> {
    use chrono::format::{self, Fixed, Item, Numeric, Pad, Parsed};

    #[derive(Default)]
    struct ParseItems(u8);

    impl Iterator for ParseItems {
        type Item = Item<'static>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0 += 1;
            match self.0 {
                1    => Some(Item::Numeric(Numeric::Year, Pad::Zero)),
                2    => Some(Item::Literal("-")),
                3    => Some(Item::Numeric(Numeric::Month, Pad::Zero)),
                4    => Some(Item::Literal("-")),
                5    => Some(Item::Numeric(Numeric::Day, Pad::Zero)),
                6    => Some(Item::Space(" ")),
                7    => Some(Item::Numeric(Numeric::Hour, Pad::Zero)),
                8    => Some(Item::Literal(":")),
                9    => Some(Item::Numeric(Numeric::Minute, Pad::Zero)),
                10   => Some(Item::Literal(":")),
                11   => Some(Item::Numeric(Numeric::Second, Pad::Zero)),
                12   => Some(Item::Fixed(Fixed::Nanosecond6)),
                13   => Some(Item::Literal(" UTC")),
                
                _    => None,
            }
        }
    }

    let mut parsed = Parsed::default();
    match format::parse(&mut parsed, s, ParseItems::default()) {
        Err(e) => Err(ParseError::failure(e, s)),
        Ok(()) => {
            parsed
                .to_datetime_with_timezone(&Utc)
                .map_err(|e| ParseError::failure(e, s))
        }
    }
}
