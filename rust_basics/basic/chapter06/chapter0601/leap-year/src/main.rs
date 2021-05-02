// 入力された年がうるう年か判断するプログラム
use std::io;
use std::io::Write;

fn main() {
    let mut year = String::new();
    print!("Please input a year to check if it is a leap year:");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<i32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year", year);
    } else {
        println!("{} is not a leap year.", year);
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}
