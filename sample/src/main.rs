use chrono::*;
fn main() {
    let time_str = "2021-01-01 23:30";
    let date = Local.datetime_from_str(time_str, "%Y-%m-%d %H:%M")
                            .unwrap();
    println!("{}", date);
}