// Cates
extern crate chrono;
extern crate time;

// Crate imports
use chrono::{NaiveDate};

const YEAR_OFFSET: i32 = 1900;

pub mod ddate;

#[allow(dead_code)]
#[derive(Debug)]
enum Calendar {
    Discordian,
}


#[derive(Debug)]
pub enum InputType {
    Iso6801,
    UnixTimestamp,
}


pub fn parse_date(raw_date: &String, input_type: InputType) -> NaiveDate {
    match input_type {
        InputType::UnixTimestamp => {
            let timestamp = time::at(time::Timespec{
                sec: raw_date.parse().expect("Could not parse timestamp"),
                nsec: 0});
            NaiveDate::from_yo(timestamp.tm_year + YEAR_OFFSET, timestamp.tm_yday as u32)
        },
        InputType::Iso6801 => NaiveDate::parse_from_str(raw_date.as_str(), "%Y-%m-%d")
                                        .expect("Could not parse date")
    }
}

