use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let ans = if (a + b) > (c + d) {
        "Left"
    } else if (a + b) < (c + d)  {
        "Right"
    } else {
        "Balanced"
    };

    println!("{}", ans);
}