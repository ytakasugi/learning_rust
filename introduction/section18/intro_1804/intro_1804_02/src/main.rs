use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

// 自前のエラー型の定義。エラーハンドリングのケースの応じてカスタマイズされる。
// ここで新たなエラーを書くことができ、元のエラーの実装に処理を委譲したり、
// その手前で何らかの処理を挟むことができます。
#[derive(Debug, Clone)]
struct DoubleError;

// エラーの生成は、それがどのように表示されるかとは別物です。
// そのため、エラーの表示スタイルに関する複雑なロジックを煩雑になるなどと気にする必要はありません
// エラーに関する余分な情報を持たせていないことに注意してください。
// どの文字列がパースに失敗したかなどを出力することは、
// その情報を保持させるようにエラーの定義を修正しない限りできません。
impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // ベクタの最初の要素を`Option(T)`から`Result<T, E>`へ変換し、
        // `Some(V)`を`Ok(V)`、`None`を`Err(err)`へマッピングする
        .ok_or(DoubleError)
        // 引数のクロージャが`Ok`を返した場合は、`2 * i`の結果を、そうでない場合は`Err`値をそのまま返す
        .and_then(|s| {
            // ベクタの最初の要素を`i32`型へ変換
            s.parse::<i32>()
                // `Err`値に関数と適用し、`Ok`値はそのままにして、`Result<T, E>`を`Result<T, F>へマッピングする
                .map_err(|_| DoubleError)
                // `Result<T, F>`を`Result<U, F>`へマッピングする
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
