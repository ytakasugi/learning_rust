use proconio::input;

fn main() {
    input!{
        n: i32,
        m: i32,
    }
    let accept = if n == m {
        "Yes"
    } else {
        "No"
    };

    println!("{}", accept);
}
