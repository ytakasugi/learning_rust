use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let ans: f64 = (a / 100.) * b;
    println!("{}", ans);
}
