use proconio::input;

fn main() {
    input!{
        n: u32,
    }
    println!(
        "{}",
        (0..n).map(|_| n.trailing_zeros())
        .min()
        .unwrap()
    )
}
