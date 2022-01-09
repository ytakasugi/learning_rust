use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;

extern crate csv;
extern crate serde_derive;

use chrono::offset::{Local, TimeZone};
use chrono::{DateTime};

type Record = (String, String, String, i64, i64, i64);

pub fn get_data() -> Result<Vec<Record>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut v = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        v.push(record);
    }
    Ok(v)
}

pub fn parse_time(time_str: &str) -> DateTime<Local> {
    Local.datetime_from_str(
        time_str,
        "%Y-%m-%d %H:%M"
    )
    .unwrap()
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}