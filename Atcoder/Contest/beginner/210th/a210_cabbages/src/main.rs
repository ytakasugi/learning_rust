use proconio::input;

fn main() {
    input! {
        n: i32,
        a: i32,
        x: i32,
        y: i32,
    }

    if n == a {
        println!("{}", n * x);
    } else if n < a {
        println!("{}", n * x);
    } else if n == a + (n - a) {
        println!("{}", (a * x) + ((n - a) * y));
    } else {
        println!("{}", ((n - a + 1) * x) + ((n - a) * y));
    }
}
