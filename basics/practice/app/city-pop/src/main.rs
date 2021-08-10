// 標準ライブラリ
use std::env;
use std::fs::File;
// 外部ライブラリ
use getopts::Options;
use serde::Deserialize;

#[derive(Debug, Eq, PartialEq, Deserialize)]
struct Row {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<u64>,
    latitude: Option<u64>,
    longitude: Option<u64>,
}


fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options] <data-path> <city>", program)));
}

fn main() {
    // プログラムに渡された引数のベクタを取得
    let args: Vec<String> = env::args().collect();
    // プログラム名を取得
    let program = &args[0];

    // 空の引数フラグを作成
    let mut opts = Options::new();
    // 引数フラグをセットする
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

    let data_path = &args[1];
    let city: &str = &args[2];

    let file = File::open(data_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    for row in rdr.deserialize() {
        let row: Row = row.unwrap();

        if row.city == city {
            println!(
                "{}, {}: {:?}",
                row.city, 
                row.country,
                row.population.expect("population count")
            );
        }
    }
}
