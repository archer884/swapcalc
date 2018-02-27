#![feature(test)]

extern crate chrono;
extern crate test;

use chrono::format::*;
use chrono::{NaiveDateTime, TimeZone, Utc};
use test::Bencher;

static DATE: &str = "2018-02-27 16:32:13.802940 UTC";

#[bench]
fn format_string(b: &mut Bencher) {
    let format = "%F %T%.f UTC";

    b.iter(|| {
        let datetime = NaiveDateTime::parse_from_str(DATE, format)
            .map(|datetime| Utc.from_local_datetime(&datetime).single().unwrap());

        test::black_box(datetime.unwrap());
    });
}

// Fun fact: I have tried two iterator implementations here, and one is significantly faster.
//
// In spite of my best guess, the following match expression contents are slower than the one in 
// use below:
//
// 1 => Some(Item::Numeric(Numeric::Year, Pad::Zero)),
// 2 => Some(Item::Literal("-")),
// 3 => Some(Item::Numeric(Numeric::Month, Pad::Zero)),
// 4 => Some(Item::Literal("-")),
// 5 => Some(Item::Numeric(Numeric::Day, Pad::Zero)),
// 6 => Some(Item::Space(" ")),
// 7 => Some(Item::Numeric(Numeric::Hour, Pad::Zero)),
// 8 => Some(Item::Literal(":")),
// 9 => Some(Item::Numeric(Numeric::Minute, Pad::Zero)),
// 10 => Some(Item::Literal(":")),
// 11 => Some(Item::Numeric(Numeric::Second, Pad::Zero)),
// 12 => Some(Item::Fixed(Fixed::Nanosecond6)),
// 13 => Some(Item::Literal(" UTC")),
//
// I would have expected this to be faster as a result of just going in order--increasing by one 
// on each iteration--but that's not the case. Additionally, the slice- and array-based options
// I have tried (all of which involve cloning) are also slower by approximately the same margin.

#[bench]
fn item_iterator(b: &mut Bencher) {

    #[derive(Default)]
    struct ParseItems(u8);

    impl Iterator for ParseItems {
        type Item = Item<'static>;

        fn next(&mut self) -> Option<Self::Item> {
            self.0 += 1;
            match self.0 {
                1 => Some(Item::Numeric(Numeric::Year, Pad::Zero)),
                3 => Some(Item::Numeric(Numeric::Month, Pad::Zero)),
                5 => Some(Item::Numeric(Numeric::Day, Pad::Zero)),
                6 => Some(Item::Space(" ")),
                7 => Some(Item::Numeric(Numeric::Hour, Pad::Zero)),
                9 => Some(Item::Numeric(Numeric::Minute, Pad::Zero)),
                11 => Some(Item::Numeric(Numeric::Second, Pad::Zero)),
                12 => Some(Item::Fixed(Fixed::Nanosecond6)),
                13 => Some(Item::Literal(" UTC")),
                2 | 4 => Some(Item::Literal("-")),
                8 | 10 => Some(Item::Literal(":")),

                _ => None,
            }
        }
    }

    b.iter(|| {
        let mut parsed = Parsed::default();
        parse(&mut parsed, DATE, ParseItems::default()).unwrap();
        test::black_box(parsed.to_datetime_with_timezone(&Utc).unwrap());
    });
}
