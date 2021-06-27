// Celsius, Kelvinタプル構造体を定義
#[derive(Debug)]
struct Celsius(f64);
#[derive(Debug)]
struct Kelvin(f64); 

impl Celsius {
    // 第一引数が`self`、`&mut self` `&self`, `Box<self>`の場合はメソッドとなる
    fn to_kelvin(self) -> Kelvin {
        // selfを通じてフィールドにアクセスできる
        Kelvin(self.0 + 273.15)
    } 

    // 第一引数が`self`系出ない場合は、関連関数になる
    fn from_kelvin(k: Kelvin) -> Self {
        Celsius(k.0 - 273.15)
    }
}

fn main() {
    let absolute_zero = Kelvin(0.0);
    let triple_point = Celsius(0.0);
    // 関連関数は`型名::関数名(引数)`で呼び出せる
    let celsius = Celsius::from_kelvin(absolute_zero);
    // メソッドは`値.関数名(引数)で呼び出せる
    let kelvin = triple_point.to_kelvin();

    // 標準出力
    println!("{:?}", celsius);
    println!("{:?}", kelvin);
}