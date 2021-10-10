use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }

    let failing = a.iter().filter(|&&x| x < p).count();

    println!("{}", failing);
}
