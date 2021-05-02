use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    let result = if a.pow(2) + b.pow(2) < c.pow(2) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", result);

}
