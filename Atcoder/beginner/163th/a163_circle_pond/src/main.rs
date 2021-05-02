use proconio::input;

fn main() {
    input!{
        // 半径
        r: f64,
    }
    let cl = r * 2.0 * std::f64::consts::PI;

    println!("{}", cl);
}
