fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);

    // `c`は破棄されメモリは開放される。
}

fn main() {
    // スタック領域に置かれた変数x
    let x = 5u32;

    // `u32`型はCopyトレイトを実装しているため、`x`を`y`にコピーする
    let y = x;
    println!("x is {}, y is {}", x, y);

    // `a`はヒープ上の整数へのポインタ
    let a = Box::new(5i32);

    // `a`は`b`へ所有権がムーブされる
    // すなわち、`a`の指すメモリ上の番地が`b`にコピーされるため
    // いずれもヒープ上の同じ値を指すポインタとなる。しかし所有権は`b`にある。
    let b = a;

    println!("b is {}", b);

    // `destroy_box`関数は、`b`の所有権を奪う
    destroy_box(b);

    // この時点でヒープメモリ上の資源は開放されているので、次の操作は解放
    // 解放済みメモリをデリファレンスすることになる。しかしそれはコンパイラが許さない。

}
