use std::io;

fn main() {
    // 変数を標準入力から受け付ける
    let mut number = String::new();
    // 標準入力を読み込む
    io::stdin().read_line(&mut number).ok();
    // 空白で分割してベクタにまとめる
    let vec: Vec<&str> = number.split_whitespace().collect();

    // `trim`は、先頭と末尾の空白が削除されたスライスを返却する
    // `parse`は、文字列スライスを別の型に変換する
    // `parse`を使用して、&str型から数値型へ変換
    let n = vec[0].trim().parse().unwrap_or(0);
    let m = vec[1].trim().parse().unwrap_or(1);

    let ans = if (n * m) % 2 == 0 {
        "Even"
    } else {
        "Odd"
    };

    println!("{}", ans);
}