use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let v = "Hello,World!".to_string();

    if s == v {
        println!("{}", "AC");
    } else {
        println!("{}", "WA");
    }
}
