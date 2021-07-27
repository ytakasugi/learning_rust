### エラーハンドリング(続き)

- コンビネータの限界

入出力と共に入力をパースすることは、非常によく行われます。 そして私がRustを使って個人的にやってきたことのほとんども、これに該当しています。 ですから、ここでは（そして、この後も） IOと様々なパースを行うルーチンを、エラーハンドリングの例として扱っていきます。

まずは簡単なものから始めましょう。 ここでのタスクは、ファイルを開き、その内容を全て読み込み、1つの数値に変換することです。 そしてそれに `2` を掛けて、結果を表示します。

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn opener<P: AsRef<Path>>(file_path: P) -> i32 {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let n: i32 = contents.trim().parse().unwrap();
    2 * n
}

fn main() {
    let doubled = opener("foobar");
    println!("{}", doubled);
}
```

（備考： `AsRef<Path>` を使ったのは、[`std::fs::File::open`](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) で使われているものと同じ境界 だからです。 ファイルパスとして、どんな文字列でも受け付けるので、エルゴノミックになります。）

ここでは3種類のエラーが起こる可能性があります：

1. ファイルを開くときの問題
2. ファイルからデータを読み込むときの問題
3. データを数値としてパースするときの問題

最初の2つの問題は、[`std::io::Error`](https://doc.rust-lang.org/std/io/struct.Error.html) 型で記述されます。 これは [`std::fs::File::open`](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) と [`std::io::Read::read_to_string`](https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string) のリターン型からわかります。 （ちなみにどちらも、以前紹介した [`Result` ](https://doc.rust-lang.org/std/result/enum.Result.html) 型エイリアスのイディオムを用いています。 `Result` 型のところをクリックすると、いま言った 型エイリアスを見たり、必然的に、中で使われている `io::Error` 型も見ることになるでしょう。） 3番目の問題は [`std::num::ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html) 型で記述されます。 特にこの `io::Error` 型は標準ライブラリ全体に *深く浸透しています* 。 これからこの型を幾度となく見ることでしょう。

まず最初に `opener` 関数をリファクタリングしましょう。 この関数を、このプログラムの他の構成要素と合成可能にするためには、上記の問題のいずれかに遭遇しても、パニック *しない* ようにしなければなりません。 これは実質的には、なにかの操作に失敗したときに、この関数が *エラーを返すべき* であることを意味します。 ここでの問題は、`opener` のリターン型が `i32` であるため、エラーの報告には全く役立たないことです。 従ってリターン型を `i32` から別の何かに変えることから始めましょう。

最初に決めるべきことは、 `Option` と `Result` のどちらを使うかです。 `Option` なら間違いなく簡単に使えます。 もし3つのエラーのどれかが起こったら、単に `None` を返せばいいのですから。 これはたしかに動きますし、 *パニックを起こすよりは良くなっています* 。 とはいえ、もっと良くすることもできます。 `Option` の代わりに、発生したエラーについての詳細を渡すべきでしょう。 ここでは *エラーの可能性* を示したいのですから、`Result<i32, E>` を使うのがよさそうです。 でも `E` を何にしたらいいのでしょうか？ 2つの *異なる* 型のエラーが起こり得ますので、これらを共通の型に変換する必要があります。 そのような型の一つに `String` があります。 この変更がコードにどんな影響を与えるか見てみましょう

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn opener<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    File::open(file_path)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
            .map_err(|err| err.to_string())
            .map(|_| contents)
        })
        .and_then(|contents| {
            contents.trim().parse::<i32>()
                .map_err(|err| err.to_string())
        })
        .map(|n| 2 * n)
}

fn main() {
    match opener("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

このコードは、やや難解になってきました。 このようなコードを簡単に書けるようになるまでには、結構な量の練習が必要かもしれません。 こういうものを書くときは *型に導かれる* ようにします。 `file_double` のリターン型を `Result<i32, String>` に変更したらすぐに、それに合ったコンビネータを探し始めるのです。 この例では `and_then`, `map`, `map_err` の、3種類のコンビネータだけを使いました。

`and_then` は、エラーを返すかもしれない処理同士を繋いでいくために使います。 ファイルを開いた後に、失敗するかもしれない処理が2つあります： ファイルから読み込む所と、内容を数値としてパースする所です。 これに対応して `and_then` も2回呼ばれています。

`map` は `Result` の値が `Ok(...)` のときに関数を適用するために使います。 例えば、一番最後の `map` の呼び出しは、`Ok(...)` の値（ `i32` 型）に `2` を掛けます。 もし、これより前にエラーが起きたなら、この操作は `map` の定義に従ってスキップされます。

`map_err` は全体をうまく動かすための仕掛けです。 `map_err` は `map` に似ていますが、 `Result` の値が `Err(...)` のときに関数を適用するところが異なります。 今回の場合は、全てのエラーを `String` という同一の型に変換する予定でした。 `io::Error` と `num::ParseIntError` の両方が `ToString` を実装していたので、 `to_string()` メソッドを呼ぶことで変換できました。

説明し終わった後でも、このコードは難解なままです。 コンビネータの使い方をマスタすることは重要ですが、コンビネータには限界もあるのです。 次は、早期リターンと呼ばれる、別のアプローチを試してみましょう。
