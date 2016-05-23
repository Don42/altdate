// Crates
extern crate chrono;
extern crate docopt;
extern crate rustc_serialize;
extern crate time;
extern crate libaltdate;

// Standard library imports

//Crate imports
use chrono::{NaiveDate, Datelike};
use docopt::Docopt;
use libaltdate::ddate;

const YEAR_OFFSET: i32 = 1900;

static VERSION: &'static str = "ddate (RUST implementaion of gnucoreutils) 0.1
Copyright (C) 2016 Marco Kaulea
License GPLv2: GNU GPL version 2 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Marco 'don' Kaulea.
";

const USAGE: &'static str = "
ddate

USAGE:
    ddate [options] [<date>]

Options:
    -h --help               Dispaly this help message and exit
    -v --version            Output version information and exit
    -d --discordian         Switch to output discordian dates. This is the default
    -t --timestamp          Date specifies a timestamp, instead of an isodate
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_date: Option<String>,
    flag_help: bool,
    flag_version: bool,
    flag_discordian: bool,
    flag_timestamp: bool,
}


#[derive(Debug)]
enum InputType {
    Iso6801,
    UnixTimestamp,
}


fn get_input_type(args: &Args) -> InputType {
    if args.flag_timestamp {
        InputType::UnixTimestamp
    } else {
        InputType::Iso6801
    }
}


fn parse_date(raw_date: &String, input_type: InputType) -> NaiveDate {
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


fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{}", VERSION);
        return;
        }

    let input_type = get_input_type(&args);

    let input_date = match args.arg_date {
        None => {
            let today = chrono::offset::local::Local::today();
            today.naive_local()
        },
        Some(raw_date) => parse_date(&raw_date, input_type),

    };
    let date = ddate::convert(input_date.ordinal0() as u16,
                   input_date.year() as i32).unwrap();
    println!("{:?}, ", date);
}

