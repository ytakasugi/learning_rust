use std::num::ParseIntError;

/*
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // `first_number_str`を`i32`型へ変換するとき・・・
    match first_number_str.parse::<i32>() {
        // `Ok(first_number)`であれば・・・
        Ok(first_number) => {
            // `second_number_str`を`i32`型へ変換するとき・・・
            match second_number_str.parse::<i32>() {
                // `Ok(second_number)`であれば・・・
                Ok(second_number) => {
                    // `first_number * second_number`
                    Ok(first_number * second_number)
                },
                // `second_number_str`を`i32`型へ変換したときに失敗した場合
                Err(e) => Err(e)
            }
        },
        // `first_number`を`i32`型へ変換したときに失敗した場合
        Err(e) => Err(e)
    }
}
*/

// 値が有効ならnを変更し、無効であればエラーをそのまま見送
fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    // `and_then`は、結果が[Ok]の場合はopを呼び出し、そうでない場合はselfの[Err]値を返す
    // `first_number_str`のパースの結果が`Ok(n)`なら、`first_number`を引数とし`{}`内の処理結果を返却するクロージャ
    first_number_str.parse::<i32>().and_then(|first_number| {
        // `map`は、含まれる[Ok]値に関数を適用し、[Err]値はそのままにすることで、Result<T, E>をResult<U, E>にマッピングする。
        // `second_number_str`のパースの結果が`Ok(n)`なら、`second_number`を引数とし`first_number * second_number`を返却するクロージャ
        // `Err`値はそのままとなる
        second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
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
