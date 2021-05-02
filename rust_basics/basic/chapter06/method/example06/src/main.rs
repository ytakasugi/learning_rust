struct Circle {
    radius: u32,
}

impl Circle {
    // Circle: diameterメソッド
    fn diameter(&self) -> u32 {
        self.radius * 2
    }

    // small_circle関連関数
    fn small_circle() -> Circle {
        Circle { radius: 1 }
    }

}

fn main() {
    let circle1 = Circle::small_circle();
    println!("Circle1's diameter: {}", circle1.diameter());
}
