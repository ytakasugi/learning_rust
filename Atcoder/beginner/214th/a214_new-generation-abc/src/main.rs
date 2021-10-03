use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    if 125 >= n {
        println!("{}", 4);
    } else if 126 <= n && n <= 211 {
        println!("{}", 6);
    } else {
        println!("{}", 8);
    }
}
