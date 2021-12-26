use std::fs::read_to_string;
use std::env;

fn run(path: String) {
    match read_to_string(path) {
        Ok(content) => println!("{}", content),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    /*match env::args().nth(1) {
        Some(path) => run(path),
        None => println!("No path is specified!"),
    }*/

    if let Some(path) = env::args().nth(1) {
        run(path)
    }
    
}
