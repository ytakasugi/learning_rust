use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(i64, i64); n],
    }

    let mut sum = 0;

    for (a, b) in &v {
        sum += b * (b + 1) / 2 - a * (a - 1) / 2;
    }

    println!("{}", sum);
}
