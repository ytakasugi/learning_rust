use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut a: Vec<_> = a.into_iter().zip(0..).collect();
    //println!("{:?}", a);
    a.sort();
    println!("{}", a[n - 1].1);
}