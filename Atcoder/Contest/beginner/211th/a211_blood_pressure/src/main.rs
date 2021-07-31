use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let avg = ((a - b) / 3.) + b;

    println!("{}", avg);
}
