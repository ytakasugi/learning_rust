// 標準ライブラリ
use std::error::Error;
use std::env;
use std::ffi::OsString;
use std::fs::File;

// 外部ライブラリ
extern crate csv;
extern crate serde;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    latitude: f64,
    longitude: f64,
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    city: String,
    state: String,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // ファイルパスを受け取る。
    let file_path = get_first_arg()?;
    // `get_first_arg`関数がエラー出なければ、ファイルを開く。
    let file = File::open(file_path)?;
    // CSVパーサーを生成し、開いたファイルからデータを読む

    let mut rdr = csv::Reader::from_reader(file);

    // 読み込んだレコードが問題なければ、そのまま標準出力する。
    // そうでなければ、エラーを`Box<Error>`に変換して返す。
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:#?}", record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none.")),
        Some(file_path) => Ok(file_path),
    }
}