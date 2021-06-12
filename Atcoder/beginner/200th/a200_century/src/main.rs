use proconio::input;

fn main() {
    input! {
        n: f64,
    }

    let ans = ((n + 99.) / 100.).floor();
    println!("{}", ans);
}
