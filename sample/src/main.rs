trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn do_something(x: &dyn Foo) {
    x.method();
}

// `Box<dyn Foo>`で記述した場合
// fn do_something(x: Box<dyn Foo>) {
//     x.method();
// }

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(&x);
    do_something(&y);

    // `Box<dyn Foo>`の場合は、以下のように記述する
    //do_something(Box::new(x));
    //do_something(Box::new(x));

    
}