fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        // 値からイテレータを作成する
        .into_iter()
        // クロージャを受け取り、各要素でクロージャを呼び出すイテレータを作成
        .map(|s| s.parse::<i32>())
        // イテレータをコレクションに変換
        .collect();
    println!("Result: {:?}", numbers);
}
