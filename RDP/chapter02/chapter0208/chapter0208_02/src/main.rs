use std::io;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let arg = "-";

    // 動的ディスパッチを得るために型を割り当てる必要がある
    let _readable: Box<dyn io::Read> = if arg == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(fs::File::open(arg)?)
    };

    Ok(())
}
