use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let patterns: Vec<Vec<char>> = ["dream", "dreamer", "erase", "eraser"]
        // イテレータを生成
        .iter()
        // 文字列スライスの`char`に対するイテレータを反転させ、コレクションに変換し、イテレータを`map`に返す
        .map(|s| s.chars().rev().collect())
        // `map`呼び出しから返却されたイテレータをコレクションに変換
        .collect();
    let s: Vec<char> = n.chars().rev().collect();
    let mut s = &s[..];
    let mut succeeded = true;
    while s.len() > 0 {
        let matched = patterns.iter().find(|&p| s.starts_with(p));
        if let Some(p) = matched {
            s = &s[p.len()..];
        } else {
            succeeded = false;
            break;
        }
    }
    println!("{}", if succeeded { "YES" } else { "NO" });
}
