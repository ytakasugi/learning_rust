use proconio::input;

fn main() {
    input!{
        d: f64,
        t: f64,
        s: f64,
    }

    let time = d / s;

    if time <= t {
        println!("Yes");
    } else {
        println!("No");
    }
}
