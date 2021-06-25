fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";

    // `?`は、Result型を返す関数で使用できる演算子であり、
    // 「その直前の結果のResult型の値がOk(t)であればtを返し、Err(e)であればErr(e)で早期リターンして関数を終了する」という動作をする。
    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    num_str // 最初は`num_str`は`&str`型
    .trim() // 文字列の前後の空白を削除する。この時点では、型は`&str`のまま
    .parse::<i32>() // `&str`を`i32`に変換する。結果はResult<i32, String>型
    .map(|t| t * 2) // `parse()の結果が`Ok(t)`の場合のみ、`t * 2`を実行して`Ok(t * 2)`となる。
    .map_err(|e| e.to_string()) // `parse()`の結果が`Err(e)の場合のみ`e`の文字列表現(`String`型)を返す
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}