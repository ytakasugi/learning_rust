use proconio::input;
fn main() {
    input! {
        n: i32,
        sum: i32,
    }

    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k == n && i * 10000 + j * 5000 + k * 1000 == sum {
                    println!("{} {} {}", i, j , k);
                    return;
                }
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}