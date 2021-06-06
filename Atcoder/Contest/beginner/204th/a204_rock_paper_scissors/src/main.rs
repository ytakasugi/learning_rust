use proconio::input;

fn main() {
    input! {
        x: i32,
        y: i32,
    }

    if x == y {
        println!("{}", x);
    } else if x == 0 && y == 1 {
        println!("{}", 2);
    } else if x == 0 && y == 2 {
        println!("{}", 1);
    } else if x == 1 && y == 0{
        println!("{}", 2);
    } else if x == 1 && y == 2 {
        println!("{}", 0);
    } else if x == 2 && y == 0 {
        println!("{}", 1);
    } else {
        println!("{}", 0);
    }
}