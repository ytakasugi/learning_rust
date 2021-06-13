use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: u32,
    }
    let c = if c % 2 == 0 {
        2
    } else {
        1
    };

    let x = a.pow(c);
    let y = b.pow(c);
    let ans = if x == y {
        "="
    } else if x > y {
        ">"
    } else {
        "<"
    };
    
    println!("{}", ans);
}