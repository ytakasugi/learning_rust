use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;


// 値が有効ならnを変更し、無効であればエラーをそのまま見送
fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    // `and_then`は、結果が[Ok]の場合はopを呼び出し、そうでない場合はselfの[Err]値を返す
    // `first_number_str`を`i32`型へ変換
    first_number_str.parse::<i32>()
        // クロージャの処理結果が`Ok`なら
        .and_then(|first_number| {
        // `map`は、含まれる[Ok]値に関数を適用し、[Err]値はそのままにすることで、Result<T, E>をResult<U, E>にマッピングする。
        // `second_number_str`を`i32`型へ変換
        second_number_str.parse::<i32>()
            // `second_number`を引数とし、`first_number * second_number`を返す
            // `Err`値の場合はそのまま
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        // 結果が`Ok(n)`なら、`println!("n is {}", n)`を実行
        Ok(n)  => println!("n is {}", n),
        // 結果が`Err(e)`なら、`println!("Error: {}", e)`を実行
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    // ここは以前と変わらず、妥当な解を与えます。
    let twenty = multiply("10", "2");
    print(twenty);

    // こちらは今度は有益なエラーメッセージを与えます。
    let e1 = multiply("2", "e");
    print(e1);
}
