use proconio::input;

fn main() {
    input!{
        n: i32,
        w: i32,
    }

    let ans = n / w;

    println!("{}", ans);
}
