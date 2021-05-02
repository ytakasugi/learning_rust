use proconio::input;

fn main() {
    input!{
        n: i32,
        a: i32,
        b: i32,
    }

    let ans = (n - a) + b;

    println!("{}", ans);

}
