use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    // 入力文字列を`Vec<char>`へ変換
    let v: Vec<char> = s.chars().collect();

    if v[n - 1] == 'o' {
        println!("Yes");
    } else {
        println!("No");
    }

}
