use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
extern crate regex;
use regex::Regex;

fn usage() {
    println!("rsgrep PATTERN FILENAME");
}

fn main() {
    // 引数からパターンを取り出す
    let pattern = match env::args().nth(1) {
        Some(pattern) => pattern,
        None => {
            usage();
            return;
        }
    };

    // 取り出したパターンから`Regex`を改めて作成する
    // 無効な正規表現だった場合などには、エラーが返る
    let reg = match Regex::new(&pattern) {
        Ok(reg) => reg,
        Err(e) => {
            println!("invalid regexp {}:{}", pattern, e);
            return;
        }
    };

    // envモジュールのargs関数で引数を取得
    // そのうち、2番目を`nth`で取得(0番目はプログラム名、1番目はパターン)
    let filename = match env::args().nth(2) {
        // あれば取り出す
        Some(filename) => filename,
        // なければヘルプを表示して終了
        None => {
            usage();
            return;
        }
    };

    // File構造体の`open`関連関数でファイルを開く
    // 失敗する可能性があるので、`Result`で返される
    // 下で`filename`を使用するため、ここでは`&filename`と参照で渡す
    let file = match File::open(&filename) {
        // 成功すれば取り出す
        Ok(file) => file,
        Err(e) => {
            println!("An error occurred while opening file {}:{}", filename, e);
            return;
        }
    };

    // Fileをそのまま使うと遅いことと、`lines`メソッドを使用するために`BufReader`に包む
    // `new`は関連関数
    let input = BufReader::new(file);
    // `BufReader`が実装するトレイトの`BufRead`にある`lines`メソッドを呼び出す
    // 返り値はイテレーターなので、`for`式で繰り返しができる
    for line in input.lines() {
        // 入力がUTF-8でないなどの理由で行のパースに失敗することがあるので、`lines`メソッドもResultに包まれている
        let line = match line {
            Ok(line) => line,
            // 失敗したらそのまま終了することにする
            Err(e) => {
                println!("An error occurred while reading a line {}", e);
                return;
            }
        };
        // パターンマッチしたら標準出力する
        // is_matchはread onlyなので、参照型で受け取る
        if reg.is_match(&line) {
            // 参照型で引数を渡しているので、ここで使用できる
            println!("{}", line);
        }
    }
}
