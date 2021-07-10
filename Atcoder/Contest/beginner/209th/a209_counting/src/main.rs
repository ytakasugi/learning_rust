use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    if a < b {
        println!("{}", b - a + 1);
    } else if a == b {
        println!("{}", 1);
    } else {
        println!("{}", 0);
    }
}
