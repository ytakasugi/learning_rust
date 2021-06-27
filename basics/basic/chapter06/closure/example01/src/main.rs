fn main() {
    let one = 1;
    let plus_one = |x| {
        x + one
    };

    println!("10 + 1 = {}", plus_one(10));
}
