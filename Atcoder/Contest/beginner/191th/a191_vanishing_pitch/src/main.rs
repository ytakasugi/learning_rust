use proconio::input;

fn main() {
    input! {
        // 速さ
        v: i32,
        // 時間1
        t: i32,
        // 時間2
        s: i32,
        // 距離
        d: i32, 
    }
    // 距離
    let a = v * (s - t) + 1;

    if a >= d {
        println!("Yes");
    } else {
        println!("No");
    }
}