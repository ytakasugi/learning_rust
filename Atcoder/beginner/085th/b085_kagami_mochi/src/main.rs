use proconio::input;

fn main() {
    input! {
        n: i32,
        mut d: [i32; n],
    }

    // 要素をソートする
    d.sort();
    // `Vec`の要素がソートされている場合、すべての重複を削除する
    d.dedup();
    println!("{}", d.len());
}
