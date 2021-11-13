use std::io;

extern crate csv;

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        let record = result.expect("a CSV record");
        let time = &record[2];
        let total = &record[4];
        println!("{}, {}", time, total);
    }
}
