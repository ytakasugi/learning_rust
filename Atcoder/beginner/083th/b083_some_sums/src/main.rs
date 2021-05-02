use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let ans = (1..=n)
        // クロージャを使用して要素を生成するかどうかを決定するイテレータを作成
        .filter(|x| {
            // `x`を`String`型に変換
            let sum = x.to_string()
                // 文字列スライスの文字数を表すイテレータへ変換
                .chars()
                // `char`を`u8`へ変換
                .map(|c| (c as u8 - b'0') as u32)
                // イテレータの要素を`u32`型として合計する
                .sum::<u32>();
            // `sum`が`a`以上`b`以上
            a <= sum && sum <= b
        })
        // `filter`が返した要素を`u32`型として合計
        .sum::<u32>();
    
    println!("{}", ans);
}
