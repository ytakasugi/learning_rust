struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 以下の関数はトレイト境界を設けているが、そのトレイトが空であるか否かとは関係がない。
fn red<T: Red>(_: &T)   -> &'static str { 
    "red" 
}

fn blue<T: Blue>(_: &T) -> &'static str { 
    "blue" 
}

fn main() {
    // 訳注: 以下は全て鳥の名前
    // 猩々紅冠鳥
    let cardinal = Cardinal;
    // アオカケス
    let blue_jay = BlueJay;
    // 七面鳥
    let _turkey   = Turkey;

    // トレイト境界のため、`red`は`blue_jay`に対しては使用できない。
    // `blue`と`Cardinal`も同様、
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}
