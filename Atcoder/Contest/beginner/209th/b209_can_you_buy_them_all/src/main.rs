use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let size = a.len() / 2;
    let mut sum = 0;

    for i in &a {
        sum += i
    }

    let price = sum - size;
    if x >= price {
        println!("Yes");
    } else {
        println!("No");
    }
}
