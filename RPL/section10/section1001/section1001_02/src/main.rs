// 構造体を定義して<>記法で1つ以上のフィールドにジェネリックな型引数を使用する
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("integer {:?}", integer);
    println!("integer {:?}", float);
    println!("integer {:?}", integer_and_float);
}
