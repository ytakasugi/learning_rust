### エラーハンドリング(続き)

- `Option`と`Result`を合成する

ここでの問題は `argv.nth(1)` が `Option` を返すのに、 `arg.parse()` は `Result` を返すことです。 これらを直接合成することはできません。 `Option` と `Result` の両方に出会ったときの *通常の* 解決策は `Option` を `Result` に変換することです。 この例で（`env::args()` が）コマンドライン引数を返さなかったということは、ユーザーがプログラムを正しく起動しなかったことを意味します。 エラーの理由を示すために、 `String` を使うこともできます。

```rust
use std::env;

fn arguments(mut argv: env::Args) -> Result<i32, String> {
    argv.nth(1)
        // `Option::ok_or`コンビネータを使用して、`Option<T>`を`Result<T, E>`に変換
        .ok_or("Please give at least one argument".to_owned())
        // `arg.parse::<i32>`が返す`Result<i32, ParseIntError`>を
        // `Result::map_err`コンビネータで`Result<i32, String>`へ変換
        .and_then(|arg| arg.parse::<i32>()
            .map_err(|err| err.to_string()))
        .map(|n| 2 * n)
}

fn main() {
    match arguments(env::args()) {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

この例では、いくつか新しいことがあります。 ひとつ目は [`Option::ok_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or) コンビネータを使ったことです。 これは `Option` を `Result` へ変換する方法の一つです。 変換には `Option` が `None` のときに使われるエラーを指定する必要があります。 他のコンビネータと同様に、その定義はとてもシンプルです。

```rust
fn ok_or<T, E>(option: Option<T>, err: E) -> Result<T, E> {
    match option {
        Some(val) => Ok(val),
        None => Err(err),
    }
}
```

ここで使った、もう一つの新しいコンビネータは [`Result::map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err) です。 これは `Result::map` に似ていますが、 `Result` 値の*エラー*の部分に対して関数をマップするところが異なります。 もし `Result` の値が `Ok(...)` だったら、そのまま変更せずに返します。

`map_err`を使った理由は、（`and_then` の用法により）エラーの型を同じに保つ必要があったからです。 ここでは（`argv.nth(1)`が返した） `Option<String>` を `Result<String, String>` に変換することを選んだため、`arg.parse()` が返した `ParseIntError` も `String` に変換しなければならなかったわけです。
