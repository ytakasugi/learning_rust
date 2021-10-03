use proconio::input;

fn main() {
    input! {
        s: isize,
        t: isize,
    }

    let mut cnt = 0;

    for a in 0..=s {
        for b in 0..=s {
            for c in 0..=s {
                if a + b + c <= s && a * b * c <= t {
                    cnt += 1;
                }
            }
        }
    }
    println!("{}", cnt);
}
