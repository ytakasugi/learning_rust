use proconio::input;


fn fact(n: usize) -> usize {
    // `n`が1より小さいとき
    if n <= 1 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn main() {
    input! {
        mut p: usize,
    }

    let mut ans = 0;

    for i in (1..10).rev() {
        let coin = fact(i);
        ans += p / coin;
        p %= coin;
    }
    println!("{}", ans)
}
