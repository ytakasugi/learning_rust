use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }

    let mut  inner_product = 0;

    for i in 0..n {
        inner_product += a[i] * b[i];
    }

    if inner_product == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
