use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    if x.min(y) + 3 > x.max(y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
