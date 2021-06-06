use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }

    let mut sum = 0;

    for &i in &a {
        if i > 10 {
            sum += i - 10
        } else {
            sum += 0
        }
    }

    println!("{}", sum);
}
