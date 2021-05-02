/*
元のエラーを維持しながらシンプルなコードを書くには、Boxしてしまうと良いでしょう。
欠点として、元のエラー型はランタイムまで判明せず、静的に決定されないことが挙げられます。
標準ライブラリはBoxに、Fromを介してあらゆるErrorトレートを実装した型から
Box<Error>トレイトオブジェクトへの変換を実装させることで、エラーをboxしやすくしてくれます。
*/

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
struct EmptyVec;

// `EmptyVec`に`Display`トレイトを実装
impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

// `EmptyVec`に`Error`トレイトを実装
impl error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&(dyn error::Error)> {
        None
    }
}


fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // `Box`に変換
        .ok_or_else(|| EmptyVec.into())
        .and_then(|s| {
            s.parse::<i32>()
                // `Box`に変換
                .map_err(|e| e.into())
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}
