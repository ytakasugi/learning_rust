use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }

    let mut max = 0;
    let mut ans = 0;

    for i in a.iter() {
        max = max.max(*i);
        ans += max - i;
    }

    println!("{}", ans);
}
