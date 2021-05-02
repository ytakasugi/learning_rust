extern crate minigrep;
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // イテレータをベクタに変換
    // `collect()`を使用するときは、コンパイラは型を推論できないないので型注釈すること
    let args: Vec<String> = env::args().collect();

    // 引数として、`args`を参照する
    // Config::newのErr値を`err`引数のクロージャに渡している
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題があった場合
        eprintln!("Problem parsing argments: {}", err);
        // 終了コード1でプロセスを終了する
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.filename);
    
    if let Err(e) = minigrep::run(config)  {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}