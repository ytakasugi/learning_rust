use std::io;
use std::num;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
enum CliError {
    Io(::std::io::Error), 
    Parse(::std::num::ParseIntError)
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError { 
        CliError::Io(err) 
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError { 
        CliError::Parse(err) 
    }
}

fn opener<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let n: i32 = contents.trim().parse()?;
    Ok(2 * n)
}

fn main() {
    match opener("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}