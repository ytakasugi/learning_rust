### [Command line apps in Rust](https://rust-cli.github.io/book/index.html#command-line-apps-in-rust)

Rustはスタティックにコンパイルされた高速な言語で、優れたツールと急速に成長するエコシステムを備えています。そのため、Rustはコマンドラインアプリケーションを書くのに最適な言語です。コマンドラインアプリケーションは、小さく、ポータブルで、素早く実行できなければなりません。コマンドラインアプリケーションは、Rustの学習を始めるのにも、チームにRustを紹介するのにも最適な方法です。

シンプルなコマンドラインインターフェイス（CLI）を使ってプログラムを書くことは、Rust言語に慣れていない初心者にとって、感触をつかむための素晴らしい練習になります。しかし、このトピックには多くの側面があり、後になって初めて明らかになることがよくあります。

この本はこのような構成になっています。簡単なチュートリアルから始まり、最後には実際に動くCLIツールを手にすることができます。Rustの中核となる概念や、CLIアプリケーションの主要な側面に触れることができます。この後の章では、これらの側面のいくつかをより詳しく説明します。

CLIアプリケーションに飛び込む前に、最後に一言。この本の中でエラーを見つけたり、もっとコンテンツを書くのを手伝いたいと思ったら、CLI WGの[リポジトリ](https://github.com/rust-cli/book)でそのソースを見つけることができます。皆様からのフィードバックをお待ちしております。

---

### Learning Rust by Writing a Command Line App in 15 Minutes

このチュートリアルでは、RustでCLI（コマンドライン・インターフェース）アプリケーションを書く方法を紹介します。プログラムが動くようになるまでには、だいたい15分くらいかかるでしょう（1.3章あたり）。その後は、小さなツールを出荷できるようになるまで、プログラムを調整していきます。

この章では、プログラムを作成するために必要なことや、より詳しい情報を得るための場所などを紹介します。今すぐ必要でない部分は読み飛ばしても構いませんし、どの時点からでも飛び込んで構いません。

> 前提条件 このチュートリアルは、プログラミングの一般的な入門書に代わるものではなく、いくつかの一般的な概念に精通していることを前提としています。また、コマンドラインやターミナルの使用に慣れている必要があります。すでに他の言語をいくつか知っている場合は、Rustに初めて触れるきっかけになるでしょう。
>
> 助けを求める。もし、Rustの機能に圧倒されたり、混乱したりした場合は、Rustに付属する広範囲な公式ドキュメントを参照してください。ほとんどのRustのインストールに付属していますし（rustup doc）、[doc.rust-lang.org](https://doc.rust-lang.org/)からオンラインで入手することもできます。
>
> また、質問をすることも大歓迎です。Rustのコミュニティはフレンドリーで親切なことで知られています。[コミュニティページ](https://www.rust-lang.org/community)では、Rustについて議論している場所のリストを見ることができます。

どんな企画を書きたいのか？まずは簡単なものから始めてみるのはどうでしょう。小さな grep クローンを書いてみましょう。これは、文字列とパスを与えると、与えられた文字列を含む行だけを表示するツールです。これを`grrs`（「grass」と発音します）と呼びましょう。

最終的には、このツールを次のように実行できるようにしたいと思います。

```
$ cat test.txt
foo: 10
bar: 20
baz: 30
$ grrs foo test.txt
foo: 10
$ grrs --help
[some help text explaining the available options]
```

> 注：この本はRust 2018用に書かれています。例題のコードはRust 2015でも使用できますが、`extern crate foo;`の呼び出しを追加するなど、少し手を加える必要があるかもしれません。
>
> Rust 1.31.0 (またはそれ以降)を実行し、`Cargo.toml`ファイルの`[package]`セクションに`edition = "2018 "`が設定されていることを確認してください。

---

### Project setup

まだインストールしていない場合は、Rustをコンピュータに[インストール](https://www.rust-lang.org/tools/install)してください（数分で完了します）。その後、ターミナルを開き、アプリケーションコードを格納するディレクトリに移動します。

プログラミングプロジェクトを保存したディレクトリで、 `cargo new grrs`を実行してみましょう。新しく作成された`grrs`ディレクトリを見ると、Rustプロジェクトの典型的なセットアップがあります。

- `Cargo.toml`ファイルは、使用する依存関係や外部ライブラリのリストなど、プロジェクトのメタデータを含んでいます。

- `src/main.rs`ファイルは、私たちの(メイン)バイナリのエントリーポイントです。

`grrs`ディレクトリで`cargo run`を実行して、"Hello World "が表示されれば、準備完了です。

#### What it might look like

```
$ cargo new grrs
     Created binary (application) `grrs` package
$ cd grrs/
$ cargo run
   Compiling grrs v0.1.0 (/Users/pascal/code/grrs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.70s
     Running `target/debug/grrs`
Hello, world!
```

---

### Parsing command line arguments

典型的なCLIツールの起動は以下のようになります。

```
grrs foobar test.txt
```

このプログラムは、`test.txt`を見て`foobar`を含む行をプリントアウトすることを期待しています。しかし、どうやってこの2つの値を得るのでしょうか？

プログラムの名前の後のテキストは、しばしば「コマンドライン引数」または「コマンドラインフラグ」と呼ばれます（特に`--this`のような形をしている場合）。内部的には、オペレーティングシステムは通常、これらを文字列のリストとして表現します。大まかに言えば、これらはスペースで区切られます。

これらの引数については、様々な考え方があり、どのように解析すればより扱いやすいものになるかということがあります。また、自分のプログラムのユーザーに、どのような引数をどのような形式で与える必要があるかを伝える必要があります。

#### Getting the arguments

標準ライブラリには、`std::env::args()`という関数があり、与えられた引数のイテレータを与えます。最初のエントリ（インデックス0）は、プログラムが呼び出されたときの名前（例：`grrs`）で、その後に続くものはユーザーがその後に書いたものです。

この方法で生の引数を取得するのは非常に簡単です（ファイル`src/main.rs`の`fn main() {`の後にあります）。

```rust
let pattern = std::env::args().nth(1).expect("no pattern given");
let path = std::env::args().nth(2).expect("no path given");
```

#### CLI arguments as data type

CLIの引数をテキストの束として考えるのではなく、プログラムへの入力を表すカスタムデータタイプとして考えることは、しばしば有益です。

grrs foobar test.txtを見てください。2つの引数があります。まずパターン（検索する文字列）、次にパス（検索するファイル）です。

これ以上何が言えるでしょうか？まず第一に、両方とも必須であるということです。デフォルト値については触れていないので、ユーザーは常に2つの値を提供することになります。さらに、その型についても少し触れておきましょう。パターンは文字列、第2引数はファイルへのパスです。

Rustでは、扱うデータを中心にプログラムを構成するのが一般的なので、このようなCLI引数の見方はとてもしっくりきます。まずはこれを見てみましょう（ファイル`src/main.rs`の`fn main() {`の前）。

```rust
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}
```

これは、データを格納する2つのフィールド、`pattern`と`path`を持つ新しい構造体（[`struct`](https://doc.rust-lang.org/1.39.0/book/ch05-00-structs.html)）を定義するものです。

> 余談ですが[`PathBuf`](https://doc.rust-lang.org/1.39.0/std/path/struct.PathBuf.html)は[`String`](https://doc.rust-lang.org/1.39.0/std/string/struct.String.html)のようなものですが、クロスプラットフォームで動作するファイルシステムのパスのためのものです。

さて、プログラムが受け取った実際の引数をこの形にする必要があります。一つの方法は、オペレーティングシステムから受け取った文字列のリストを手動で解析し、自分で構造を構築することです。それは次のようなものです。

```rust
let pattern = std::env::args().nth(1).expect("no pattern given");
let path = std::env::args().nth(2).expect("no path given");
let args = Cli {
    pattern: pattern,
    path: std::path::PathBuf::from(path),
};
```

これは動作しますが、あまり便利ではありません。`--pattern="foo"`や`--pattern "foo" `をサポートしなければならない場合、どのように対処しますか？`--help`の実装はどうしますか？

#### Parsing CLI arguments with StructOpt

より良い方法は、多くの利用可能なライブラリの1つを使用することです。コマンドライン引数を解析する最も有名なライブラリは[`clap`](https://clap.rs/)と呼ばれています。[`clap`](https://clap.rs/)は、サブコマンドのサポート、シェルの補完、優れたヘルプ・メッセージなど、期待されるすべての機能を備えています。

[`structopt`](https://docs.rs/structopt)ライブラリは[`clap`](https://clap.rs/)をベースにしており、構造体の定義に対して[`clap`](https://clap.rs/)のコードを生成する「derive」マクロを提供しています。これは非常に素晴らしいことです。構造体に注釈を付けるだけで、引数を解析してフィールドに入れるコードを生成してくれます。

まず、`Cargo.toml`ファイルの`[dependencies]`セクションに`structopt = "0.3.13 "`を追加して、`structopt`をインポートしましょう。

これで、コードの中で`use structopt::StructOpt;`と書き、`struct Cli`の上に `#[derive(StructOpt)]`を追加することができます。ついでに、ドキュメント用のコメントも書いておきましょう。

以下のようになります（ファイル`src/main.rs`の`fn main() {`の前）。

```rust
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
```

> 注：フィールドに追加できるカスタム属性はたくさんあります。たとえば、`PathBuf`タイプをどのように解析するかを`structopt`に伝えるための属性を追加しました。このフィールドを`-o`または`--output`の後の引数に使用するには、`#[structopt(short = "o", long = "output")]`を追加します。詳細については、`structopt`のドキュメントを参照してください。

`Cli`構造体のすぐ下には、テンプレートのメイン関数があります。プログラムが開始されると、この関数が呼び出されます。最初の行は・・・

```rust
fn main() {
    let args = Cli::from_args();
}
```

これは、引数を解析して`Cli`構造体を作成しようとします。

しかし、それが失敗したらどうしますか？それがこのアプローチの良いところです。`Clap`はどのフィールドを期待しているのか、そしてその期待されるフォーマットは何かを知っています。また、--helpメッセージを自動的に生成したり、`--putput`と書いたのに`--output`を渡してしまったというような素晴らしいエラーを出すこともできます。

> 注意：`from_args`メソッドは、main関数の中で使用することを想定しています。失敗した場合は、エラーやヘルプのメッセージを出力し、すぐにプログラムを終了します。他の場所では使用しないでください。

#### This is what it may look like

引数なしで実行します。

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 10.16s
     Running `target/debug/grrs`
error: The following required arguments were not provided:
    <pattern>
    <path>

USAGE:
    grrs <pattern> <path>

For more information try --help
```

`cargo run`を直接使用する際には、`--`の後に引数を記述することで、引数を渡すことができます。

```
$ cargo run -- some-pattern some-file
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/grrs some-pattern some-file`
```

ご覧の通り、出力はありません。これは良いことです。これはエラーがなく、プログラムが終了したことを意味します。

> **Exercise for the reader:** このプログラムが引数を出力するようにしてください

---

### First implementation of grrs

前回のコマンドライン引数の章の後、入力データができましたので、実際のツールを書き始めることができます。今、私たちのメイン関数にはこの行しかありません。

```rust
let args = Cli::from_args();
```

まずは、入手したファイルを開いてみましょう。

```rust
let content = std::fs::read_to_string(&args.path)
    .expect("could not read file");
```

> 余談ですが。ここにある[`.expect`](https://doc.rust-lang.org/1.39.0/std/result/enum.Result.html#method.expect)メソッドを見てください。これはquitのショートカット関数で、値（ここでは入力ファイル）を読み込めなかったときにプログラムを直ちに終了させるものです。次の[Nicer error reporting](https://rust-cli.github.io/book/tutorial/errors.html)の章では、これを改善する方法を見ていきます。

では、行を繰り返して、パターンを含む行をそれぞれ表示してみましょう。

```rust
for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
}
```

`cargo run -- main src/main.rs`が動作するようになりました。

> **Exercise for the reader:** これは最良の実装ではありません。ファイルがどんなに大きくても、ファイル全体をメモリに読み込んでしまいます。これを最適化する方法を見つけてください! `Read_to_string()`の代わりに`BufReader`を使用するのも一案です）。

---

### Nicer error reporting

私たちは、エラーが発生するという事実を受け入れるしかありません。そして、他の多くの言語とは対照的に、Rustを使っているときにこの現実に気づかず、対処しないことは非常に困難です。Rustには例外がないので、ありとあらゆるエラー状態が関数の戻り値にエンコードされています。

#### Results

[`read_to_string`](https://doc.rust-lang.org/1.39.0/std/fs/fn.read_to_string.html)のような関数は、文字列を返しません。代わりに、文字列か何らかの型のエラー（ここでは[`std::io::Error`](https://doc.rust-lang.org/1.39.0/std/io/type.Result.html)）を含む[`Result`](https://doc.rust-lang.org/1.39.0/std/result/index.html)を返します。

どちらかを知るにはどうすればよいでしょうか。`Result`は列挙型なので、`match`を使ってどの種類なのかを確認することができます。

```rust
let result = std::fs::read_to_string("test.txt");
match result {
    Ok(content) => { println!("File content: {}", content); }
    Err(error) => { println!("Oh noes: {}", error); }
}
```

> Aside: enumとは何か、Rustでどのように機能するのかよくわからない？Rustの本のこの章を読めば、すぐに理解できます。

#### Unwrapping

さて、ファイルの内容にアクセスすることができましたが、`mutch`ブロックの後では実際には何もできません。そのためには、エラーケースをなんとか処理する必要があります。課題は、`mutch`ブロックのすべてのアームが同じ型のものを返さなければならないことです。しかし、それを回避するための巧妙なトリックがあります。

```rust
let result = std::fs::read_to_string("test.txt");
let content = match result {
    Ok(content) => { content },
    Err(error) => { panic!("Can't deal with {}, just exit here", error); }
};
println!("file content: {}", content);
```

`mutch`ブロックの後のコンテンツで`String`を使用することができます。`result`がエラーになった場合、`String`は存在しません。しかし、コンテンツを使用するポイントに到達する前にプログラムが終了するので、問題ありません。

思い切ったことをしているように見えますが、これはとても便利です。プログラムがそのファイルを読む必要があり、ファイルが存在しないと何もできない場合、終了することは有効な戦略です。`Results`には、`unwrap`というショートカットの方法もあります。

```rust
let content = std::fs::read_to_string("test.txt").unwrap();
```

#### No need to panic

もちろん、プログラムを中止することだけがエラーへの対処法ではありません。`panic!`の代わりに、簡単に`return`を書くこともできます。

```rust
let result = std::fs::read_to_string("test.txt");
let _content = match result {
    Ok(content) => { content },
    Err(error) => { return Err(error.into()); }
};
```

しかし、これにより、この関数が必要とする戻り値の型が変わります。確かに、今までの例には何かが隠されていました。それは、このコードが組み込まれている関数のシグネチャです。そして、この最後の`return`を伴う例では、それが重要になります。これがその例です。

> Aside:なぜ`return Ok(());`と書かないのでしょうか？簡単に書けますが、これも全く問題ありません。Rustでは、ブロックの最後の表現は戻り値であり、不要な戻り値は省くのが通例です。

#### Question Mark

`.unwrap()`を呼ぶことが、エラーアームの中で`panic!`とマッチするためのショートカットであるように、エラーアームの中で返ってくるマッチのための別のショートカットがあります： ?

その通り、`?`です。この演算子を`Result`型の値に追加すると、Rustは内部的に先ほど書いたマッチとよく似たものに展開します。

ぜひ試してみてください。

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("test.txt")?;
    println!("file content: {}", content);
    Ok(())
}
```

とても簡潔ですね。

> 余談ですが ここではさらにいくつかのことが起こっていますが、これを扱うのに理解する必要はありません。例えば、メイン関数のエラータイプは`Box<dyn std::error::Error>`です。しかし、`read_to_string`が`std::io::Error`を返すことを上で見てきました。これは、エラータイプを変換するコードに`?`が展開されるのでうまくいきます。
>
> `Box<dyn std::error::Error>`も面白い型です。これは、標準的なエラートレイトを実装した任意の型を含むことができるボックスです。つまり、基本的にすべてのエラーをこのボックスに入れることができるので、`?`を使用できます。`Result`を返す通常の関数のすべてで。

#### Providing Context

メインの関数に`?`を使ったときに出るエラーは、大丈夫ですが、素晴らしいものではありません。例えば、以下のようなものです。`std::fs::read_to_string("test.txt")?`を実行したが、`test.txt`というファイルが存在しなかった場合、次のような出力が得られます。

```
Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

コードにファイル名が含まれていない場合、どのファイルが`NotFound`だったのかを判断するのは非常に困難です。この問題に対処する方法は複数あります。

例えば、独自のエラータイプを作成し、それを使って独自のエラーメッセージを作成することができます。

```rust
#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    println!("file content: {}", content);
    Ok(())
}
```

これを実行すると、独自のエラーメッセージが表示されます。

```rust
Error: CustomError("Error reading `test.txt`: No such file or directory (os error 2)")
```

あまりきれいではありませんが、後から簡単にデバッグ出力を自分のタイプに合わせることができます。

このパターンは実際には非常に一般的です。しかし、これには1つの問題があります。元のエラーは保存せず、その文字列表現のみを保存するのです。よく使われる`anyhow`ライブラリには、そのためのきちんとしたソリューションがあります。`CustomError`タイプと同様に、その`Context`トレイトを使って説明を追加することができます。さらに、元のエラーも保持されるので、根本的な原因を指摘するエラーメッセージの「連鎖」を得ることができます。

まず、`Cargo.toml`ファイルの`[dependencies]`セクションに`anyhow = "1.0"`を追加して`anyhow`クレートをインポートしましょう。

完全な例は次のようになります。

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}
```

エラーが表示されます。

```rust
Error: could not read file `test.txt`

Caused by:
    No such file or directory (os error 2)
```

---

