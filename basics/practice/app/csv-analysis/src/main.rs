// 標準ライブラリ
use std::process;

use csv_analysis::run;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
