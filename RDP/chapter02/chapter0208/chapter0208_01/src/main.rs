use std::io;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let arg = "-";

    // `readable`より長く生存しないといけないため、最初に宣言する
    let (mut stdin_read, mut file_read);

    // 動的ディスパッチを得るために型を割り当てる必要がある
    let _readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };

    Ok(())
}
