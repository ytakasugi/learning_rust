use proconio::input;

fn main() {
    input! {
        n: usize,
        d: f64,
        v: [(f64, f64); n],
    }

    // `count`の初期化
    let mut count = 0;

    for &(x, y) in &v {
        if x.hypot(y) <= d {
            count += 1
        }
    }

    println!("{}", count);

}
