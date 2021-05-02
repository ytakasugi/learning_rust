use proconio::input;

fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let result = 
        if (a <= b) && (c <= 0 && d > 0) {
            b - c
        } else if (a <= b) && (c < 0 && d > 0) {
            b - c
        } else if (a <= b) && (c > 0 && d <= 0) {
            b - d            
        } else if (a <= b) && (c > 0 && d < 0) {
            b - d
        } else if (a >= b) && (c <= 0 && d > 0) {
            a - c
        } else if (a >= b) && (c <= 0 && d > 0) {
            a - c            
        } else if (a >= b) && (c > 0 && d <= 0)  {
            a - d
        } else if (a >= b) && (c > 0 && d < 0) {
            a - d
        } else {
            b - d
        };

    println!("{}", result);
}