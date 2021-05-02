use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n],
    }

    a.sort_by(|x, y| x.cmp(y).reverse());
    let mut alice = 0;
    let mut bob = 0;
    for (i, &x) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice += x;
        } else {
            bob += x;
        }
    }
    println!("{}", alice - bob);
}
