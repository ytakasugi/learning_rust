use proconio::input;

fn main() {
    input!{
        x: i32, 
    }

    let output = if x == 0 {
        "1"
    } else {
        "0"
    };

    println!("{}", output);
}
