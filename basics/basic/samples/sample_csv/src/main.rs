use std::error::Error;
use std::io;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    // CSVリーダーを構築し、各レコードを反復処理する。
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // イテレータからは`Result<StringRecord, Error>`が得られるので、ここでエラーを確認します。
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
