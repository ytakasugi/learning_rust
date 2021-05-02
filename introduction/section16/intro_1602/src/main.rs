struct Sheep{}
struct Cow{}

trait Animal {
    // インスタンスメソッドシグネチャ
    fn noise(&self) -> &'static str;
}

// `Sheep`構造体に`Animal`トレイトを実装する
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah"
    }
}

// `Cow`構造体に`Animal`トレイトを実装する
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo"
    }
}

// `Animal`を実装した構造体を返しますが、コンパイル時にどの構造体かはわからない
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);

    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
