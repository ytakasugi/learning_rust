use proconio::input;

fn main() {
    input!{
        n: i32,
    }

    // powメソッドを使用してnの2乗と3乗を計算する
    let ans = n + n.pow(2) + n.pow(3);
    println!("{}", ans);
}
