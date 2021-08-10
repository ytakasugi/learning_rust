// 標準ライブラリ
use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io;
// 外部ライブラリ
extern crate serde;

use getopts::Options;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Row {
    #[serde(rename = "Country")]
    country: String,
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "AccentCity")]
    accent_city: String,
    #[serde(rename = "Region")]
    region: String,
    #[serde(rename = "Population")]
    #[serde(deserialize_with = "csv::invalid_option")]
    population: Option<u64>,
    #[serde(rename = "Latitude")]
    latitude: Option<f64>,
    #[serde(rename = "Longitude")]
    longitude: Option<f64>,
}

struct PopulationCount {
    city: String,
    country: String,
    count: u64,
}

fn search<P: AsRef<Path>>(file_path: &Option<P>, city: &str) -> Result<Vec<PopulationCount>, Box<dyn Error + Send + Sync>> {
    let mut found = vec![];
    let input: Box<dyn io::Read> = match *file_path {
        None => Box::new(io::stdin()),
        // 所有権をムーブさせるのではなく、借用する
        Some(ref file_path) => Box::new(File::open(file_path)?),
    };
    let mut rdr = csv::Reader::from_reader(input);

    for row in rdr.deserialize() {
        let row: Row = row?;

        match row.population {
            None => {},
            Some(count) => if row.city == city {
                found.push(PopulationCount {
                    city: row.city,
                    country: row.country,
                    count: count,
                });
            },
        }
    }
    if found.is_empty() {
        Err(From::from("No matching cities with a population where found."))
    } else {
        Ok(found)
    }
}

fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options] <city>", program)));
}

fn main() {
    // プログラムに渡された引数のベクタを取得
    let args: Vec<String> = env::args().collect();
    // プログラム名を取得
    let program = &args[0];

    // 空の引数フラグを作成
    let mut opts = Options::new();
    // 引数フラグをセットする
    // 引数を取るオプションを作成
    opts.optopt("f", "file", "Choose an input file, instead of using STDIN.", "NAME");
    // 引数を取らないオプションを作成
    opts.optflag("h", "help", "Show this usage message.");

    // 引数をパースする
    let matches = match opts.parse(&args[1..]) {
        // パース成功したら、オブジェクトを取り出す
        Ok(m) => {
            m
        },
        // パースに失敗したらpanicさせる
        Err(e) => {
            std::panic::panic_any(e.to_string())
        }
    };

    // `Option`にマッチした場合は、trueを返す。
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let file = matches.opt_str("f");
    let data_path = &file.as_ref().map(Path::new);

    let city = if !matches.free.is_empty() {
        &matches.free[0] 
    } else {
        print_usage(&program, opts);
        return;
    };

    match search(data_path, city) {
        Ok(pops) => {
            for pop in pops {
                println!("{}, {}: {:?}", pop.city, pop.country, pop.count);
            }
        }
        Err(err) => println!("{}", err)
    }
}
