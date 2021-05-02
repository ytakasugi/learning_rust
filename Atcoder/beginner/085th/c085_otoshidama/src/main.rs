use proconio::input;

fn main() {
    input! {
        n: i32,
        y: i32,
    }

    let mut ans = None;

    'outer: for i in 0..=n {
        for j in 0..=n - i {
            let k = n - i - j;
            if i * 10000 + j * 5000 + k * 1000 == y {
                ans = Some((i, j, k));
                break 'outer;
            }
        }
    }
    let (x, y, z) = ans.unwrap_or((-1, -1, -1));
    println!("{} {} {}", x, y, z);
}
