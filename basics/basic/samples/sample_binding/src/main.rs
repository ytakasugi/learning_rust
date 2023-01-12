use std::fs::File;
use std::io::prelude::*;

fn binding(path: &str) -> String {
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    let s = "    AND LIMIT 100";
    
    contents + s
}

fn main() {
    println!("{}", binding("find.sql").as_str());
}
