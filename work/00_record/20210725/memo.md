### エラーハンドリング(続き)

- `Option<T>` 値を合成する

もう一つの共通のパターンは、`Option` の値が `None` だったときのデフォルト値を与えることです。 例えばファイルの拡張子がないときは、それを `rs` とみなすようなプログラムを書きたくなるかもしれません。 ご想像の通り、このような場合分けはファイルの拡張子に特有のものではありません。 どんな `Option<T>` でも使えるでしょう：

```rust
fn unwrap_or<T>(option: Option<T>, default: T) -> T {
    match option {
        None => default,
        Some(value) => value,
    }
}
```

上記の `map` と同じように、標準ライブラリの実装はただの関数ではなくメソッドになっています。

ここでの仕掛けは、`Option<T>` に入れる値と同じ型になるよう、デフォルト値の型を制限していることです。 これを使うのは、すごく簡単です。

```rust
// 文字列から文字を検索する関数
// 文字列スライスとcharを引数にとり、`Option`型を返す
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}

fn main() {
    assert_eq!(extension("foobar.csv").unwrap_or("rs"), "csv");
    assert_eq!(extension("foobar").unwrap_or("rs"), "rs");
}
```

もうひとつ特筆すべきコンビネータがあります。それは `and_then` です。これを使うと *不在の可能性* を考慮しながら、別々の処理を簡単に組み合わせることができます。 例えば、この節のほとんどのコードは、与えられたファイル名について拡張子を見つけだします。 そのためには、まずファイル *パス* から取り出したファイル名が必要です。 大抵のパスにはファイル名がありますが、 *全て* がというわけではありません。 例えば `.`, `..`, `/` などは例外です。

つまり、与えられたファイル *パス* から拡張子を見つけ出せるか、トライしなければなりません。 まず明示的な場合分けから始めましょう

```rust
# fn extension(file_name: &str) -> Option<&str> { None }
fn file_path_ext_explicit(file_path: &str) -> Option<&str> {
    match file_name(file_path) {
        None => None,
        Some(name) => match extension(name) {
            None => None,
            Some(ext) => Some(ext),
        }
    }
}

fn file_name(file_path: &str) -> Option<&str> {
#  // implementation elided
  // 実装は省略
  unimplemented!()
}
```

場合分けを減らすために単に `map` コンビネータを使えばいいと思うかもしれませんが、型にうまく適合しません。

`map` 関数は `Option<_>` 内で `extension` 関数が返した値をラップしていますが `extension` 関数自身が `Option<&str>` を返すので、式 `file_name(file_path).map(|x| extension(x))` は実際は `Option<Option<&str>>` を返すのです。

しかしながら `file_path_ext` は（`Option<Option<&str>>` ではなく）ただの `Option<&str>` を返すのでコンパイルエラーとなるのです。

そして関数が返した値は *必ず* `Some` でラップされ直します] 。 つまりこの代わりに、 `map` に似た、しかし新たに `Option<_>` で包まずに直接呼び出し元に常に `Option<_>` を返すものが必要です。

これの汎用的な実装は `map` よりもシンプルです

```rust
fn and_then<F, T, A>(option: Option<T>, f: F) -> Option<A>
        where F: FnOnce(T) -> Option<A> {
    match option {
        None => None,
        Some(value) => f(value),
    }
}
```

明示的な場合分けを省くように、 `file_path_ext` を書き直しましょう

```rust
fn file_path_ext(file_path: &str) -> Option<&str> {
    file_name(file_path).and_then(extension)
}
```

---

- `Result`型

`Result`型は、標準ライブラリで以下のように定義されている。

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 型は `Option` 型の豪華版です。 `Option` のように *不在* の可能性を示す代わりに、`Result` は *エラー* になる可能性を示します。 通常 *エラー* は、なぜ処理が実行に失敗したのかを説明するために用いられます。 これは厳密には `Option` をさらに一般化した形式だといえます。 以下のような型エイリアスがあるとしましょう。 これは全てにおいて、本物の `Option<T>` と等しいセマンティクスを持ちます。

```rust
type Option<T> = Result<T, ()>;
```

これは `Result` の2番目の型パラメータを `()` （「ユニット」または「空タプル」と発音します）に固定したものです。 `()` 型のただ一つの値は `()` です。 （そうなんです。型レベルと値レベルの項が、全く同じ表記法を持ちます!）

`Result` 型は、処理の結果がとりうる2つの可能性のうち、1つを表すための方法です。 慣例に従い、一方が期待されている結果、つまり「`Ok`」となり、もう一方が予想外の結果、つまり「`Err`」になります。

`Option` と全く同じように、`Result` 型も標準ライブラリで `unwrap` メソッドが定義されています。

- 整数をパースする

Rustの標準ライブラリを使うと、文字列を整数に変換することが、すごく簡単にできます。 

```rust
fn double_number(s: &str) -> i32 {
    2 * s.parse::<i32>().unwrap()
}

fn main() {
    let n: i32 = double_number("10");
    assert_eq!(n, 20);
}
```

上記は、文字列が数字としてパースできなければ、パニックが起こります。

```
thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }', /home/rustbuild/src/rust-buildbot/slave/beta-dist-rustc-linux/build/src/libcore/result.rs:729
```

これは少し目障りです。 もしあなたが使っているライブラリの中でこれが起こされたら、イライラするに違いありません。 代わりに、私たちの関数の中でエラーを処理し、呼び出し元にどうするのかを決めさせるべきです。 そのためには、`double_number` の戻り値の型（リターン型）を変更しなければなりません。 でも、一体何に？ ええと、これはつまり、標準ライブラリの [`parse` メソッド](https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/1.9/ja/std/primitive.str.html#method.parse) のシグネチャを見ろということです。

```rust
impl str {
    fn parse<F: FromStr>(&self) -> Result<F, F::Err>;
}
```

最低でも `Result` を使わないといけないことはわかりました。 もちろん、これが `Option` を戻すようにすることも可能だったでしょう。 結局のところ、文字列が数字としてパースできたかどうかが知りたいわけですよね？ それも悪いやり方ではありませんが、実装の内側では *なぜ* 文字列が整数としてパースできなかったを、ちゃんと区別しています。 （空の文字列だったのか、有効な数字でなかったのか、大きすぎたり、小さすぎたりしたのか。） 従って、`Result` を使ってより多くの情報を提供するほうが、単に「不在」を示すことよりも理にかなっています。 今後、もし `Option` と `Result` のどちらを選ぶという事態に遭遇したときは、このような理由付けのやり方を真似てみてください。 もし詳細なエラー情報を提供できるのなら、多分、それをしたほうがいいでしょう。 （後ほど別の例もお見せます。）

それでは、リターン型をどう書きましょうか？ 上の `parse` メソッドは一般化されているので、標準ライブラリにある、あらゆる数値型について定義されています。 この関数を同じように一般化することもできますが（そして、そうするべきでしょうが）、今は明快さを優先しましょう。 `i32` だけを扱うことにしますので、それの [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)の実装がどうなっているか探しましょう。 （ブラウザで `CTRL-F` を押して「FromStr」を探します。） そして [関連型(associated type)](https://doc.rust-lang.org/std/str/trait.FromStr.html#associated-types)から `Err` を見つけます。 こうすれば、具体的なエラー型が見つかります。 この場合、それは [`std::num::ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html)です。 これでようやく関数を書き直せます：

```rust
use std::num::ParseIntError;

fn double_number(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(2 * n),
        Err(e) => Err(e),
    }
}

fn main() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

少し良くなりましたが、たくさんのコードを書いてしまいました！ 場合分けに、またしてもやられたわけです。

コンビネータに助けを求めましょう！ ちょうど `Option` と同じように `Result` にもたくさんのコンビネータが、メソッドとして定義されています。 `Result` と `Option` の間では、共通のコンビネータが数多く存在します。 例えば `map` も共通なものの一つです

```rust
use std::num::ParseIntError;

fn double_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(|n| 2 * n)
}

fn main() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

`Result` でいつも候補にあがるのは `unwrap_or`と`and_then`です。 さらに `Result` は2つ目の型パラメータを取りますので、エラー型だけに影響を与える`map_err`（`map` に相当）と`or_else`（`and_then` に相当）もあります。

- `Result`型エイリアスを用いたイディオム

標準ライブラリでは `Result<i32>` のような型をよく見ると思います。 でも、待ってください。 2つの型パラメータを取るように `Result` を定義したはずです。 どうして、1つだけを指定して済んだのでしょう？ 種を明かすと、`Result` の型エイリアスを定義して、一方の型パラメータを特定の型に *固定* したのです。 通常はエラー型の方を固定します。 例えば、先ほどの整数のパースの例は、こう書き換えることもできます。

```rust
use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, ParseIntError>;

fn double_number(s: &str) -> Result<i32> {
    s.parse::<i32>().map(|n| 2 * n)
}

fn main() {
    match double_number("10") {
        Ok(n) => assert_eq!(n, 20),
        Err(e) => println!("Error: {:?}", e),
    }
}
```

なぜ、こうするのでしょうか？ もし `ParseIntError` を返す関数をたくさん定義するとしたら、常に `ParseIntError` を使うエイリアスを定義したほうが便利だからです。 こうすれば、同じことを何度も書かずに済みます。

標準ライブラリで、このイディオムが際立って多く使われている場所では、[`io::Result`](https://doc.rust-lang.org/std/io/type.Result.html)を用いています。 それらは通常 `io::Result<T>` のように書かれ、`std::result` のプレーンな定義の代わりに `io` モジュールの型エイリアスを使っていることが、明確にわかるようになっています。


