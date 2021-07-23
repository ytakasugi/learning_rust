struct Adder;

impl Adder {
    fn add(x: u8, y: u8, f: impl Fn(u8, u8) -> u8) -> u8 {
        f(x, y)
    }
}

fn main() {
    let arith_adder = |x, y| x + y;
    let bool_adder = |x, y| {
        if x == 1 || y == 1 {
            1
        } else {
            0
        }
    };

    let custom_adder = |x, y| 2 * x + y;

    println!("{}", Adder::add(4, 5, arith_adder));
    println!("{}", Adder::add(0, 1, bool_adder));
    println!("{}", Adder::add(1, 3, custom_adder));
}
