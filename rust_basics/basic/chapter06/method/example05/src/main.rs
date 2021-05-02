struct Circle {
    radius: u32,
}

impl Circle {
    fn diameter(&self) -> u32 {
        self.radius * 2
    }
}

fn main() {
    let circle1 = Circle { radius: 10 };
    println!("Circle1's diameter: {}", circle1.diameter());
}
