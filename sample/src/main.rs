use proconio::input;
fn main() {
    input! {
        n: i32,
        sum: i32,
    }

    for i in 0..=n {
        for j in 0..=n {
            let z = n - i - j;
            if z >= 0 && i * 10000 + j * 5000 + z * 1000 == sum {
                println!("{} {} {}", i, j, z);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}