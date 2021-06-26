use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    let mut v = vec![a, b, c];
    v.sort();

    let ans = v[1] + v[2];
    println!("{}", ans);
}
