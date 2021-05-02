use proconio::input;

fn main() {
    input!{
        n: i32,
        m: i32,
    }

    let f = (2 * n + 100) - m;

    println!("{}", f);
}
