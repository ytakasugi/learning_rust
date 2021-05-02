### [Destructor](https://doc.rust-jp.rs/rust-by-example-ja/scope/raii.html#destructor)

Rustのデストラクタの概念は Dropトレイトで提供されています。デストラクタはリソースがスコープ外になったときに呼び出されます。この特性はすべての型に実装する必要はありませんが、独自のデストラクタロジックが必要な場合にのみ実装してください。

以下の例を実行して、Dropトレイト がどのように動作するかを確認してください。メイン関数の変数がスコープ外になると、カスタムデストラクタが呼び出されます。

```rust
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}

```
