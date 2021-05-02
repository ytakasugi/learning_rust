use proconio::input;

fn main() {
    input! {
        n: i32,
    }

    let mut ans = 0;

    for i in 1..n {
        ans += (n - 1) / i;
    }

    println!("{}", ans);
}
