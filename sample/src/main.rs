fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        factorial(n - 1) * n
    }
}

fn main() {
    println!("{}", factorial(3));
}