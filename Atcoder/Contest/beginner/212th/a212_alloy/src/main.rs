use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    if a > 0 && b == 0 {
        println!("Gold");
    } else if a == 0 && b > 0 {
        println!("Silver");
    } else {
        println!("Alloy");
    }
}
