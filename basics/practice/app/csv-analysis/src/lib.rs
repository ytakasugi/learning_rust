// 標準ライブラリ
use std::error::Error;
use std::io;
// 外部ライブラリ
extern crate csv;

pub fn run() -> Result<(), Box<dyn Error>> {
    // CSVパーサーを生成し、stdinからデータを読む
    let mut rdr = csv::Reader::from_reader(io::stdin());

    
    // 読み込んだレコードが問題なければ、そのまま標準出力する。
    // そうでなければ、エラーを`Box<Error>`に変換して返す。
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}