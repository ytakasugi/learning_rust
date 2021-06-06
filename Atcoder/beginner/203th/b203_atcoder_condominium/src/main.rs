use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let mut s = 0;

    for i in 1..=n {
        for j in 1..=k {
            s += (100 * i) + j
        }
    }
    println!("{}", s);
}
