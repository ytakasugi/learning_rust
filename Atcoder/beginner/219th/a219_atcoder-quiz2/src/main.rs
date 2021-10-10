use proconio::input;

fn main() {
    input! {
        x: u32,
    }

    if x >= 90 {
        println!("expert");
    } else if 90 > x && x >= 70 {
        println!("{}", 90 - x);
    } else if 70 > x && x >= 40 {
        println!("{}", 70 - x);
    } else {
        println!("{}", 40 - x);
    }
}
