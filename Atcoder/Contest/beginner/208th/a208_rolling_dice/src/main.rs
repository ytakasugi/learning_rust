use proconio::input;

fn main() {
    input!{
        a: usize,
        b: usize,
    }

    if (a * 6) >= b && a <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
