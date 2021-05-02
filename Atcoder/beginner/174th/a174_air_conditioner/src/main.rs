use proconio::input;

fn main() {
    input!{
        x: i32,
    }

    if x >= 30 {
        println!("Yes");
    } else if x < 30 {
        println!("No");
    }
}
