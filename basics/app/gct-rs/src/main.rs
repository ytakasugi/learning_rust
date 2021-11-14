use std::process;

use gct_rs::run;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
