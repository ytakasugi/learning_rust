use proconio::input;

fn main() {
    // 標準入力用
    input!{
        n: i32,
        // 1度に作れるたこ焼きの最大個数
        x: i32,
        // 時間
        t: i32,
    }
    // たこ焼きを焼く回数を求める
    let times = (n + x - 1) / x;
    // 答えを求める
    let ans = times * t;
    // 標準出力
    println!("{}", ans);
}
