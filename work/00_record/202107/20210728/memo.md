### エラーハンドリング(続き)

- 早期リターン

前の節で使ったコードを、 *早期リターン* を使って書き直してみようと思います。 早期リターンとは、関数の途中で抜けることを指します。 `file_double` のクロージャの中にいる間は、早期リターンはできないので、明示的な場合分けまでいったん戻る必要があります。

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn opener<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        return Err(err.to_string());
    }
    let n: i32 = match contents.trim().parse() {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(2 * n)
}

fn main() {
    match opener("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {}", err),
    }
}
```

このコードが、コンビネータを使ったコードよりも良くなったのかについては、人によって意見が分かれるでしょう。 でも、もしあなたがコンビネータによるアプローチに不慣れだったら、このコードのほうが読みやすいと思うかもしれません。 ここでは明示的な場合分けを `match` と `if let` で行っています。 もしエラーが起きたら関数の実行を打ち切って、エラーを（文字列に変換してから）返します。

でもこれって逆戻りしてませんか？ 以前は、エラーハンドリングをエルゴノミックにするために、明示的な場合分けを減らすべきだと言っていました。 それなのに、今は明示的な場合分けに戻ってしまっています。 すぐにわかりますが、明示的な場合分けを減らす方法は *複数* あるのです。 コンビネータが唯一の方法ではありません。

---

- `try!`マクロ

#### ※2018Editionでは、`try!`マクロは非推奨となっており、`?`キーワードを使用する

Rustでのエラー処理の基礎となるのは `try!` マクロです。 `try!` マクロはコンビネータと同様、場合分けを抽象化します。 しかし、コンビネータと異なるのは *制御フロー* も抽象化してくれることです。 つまり、先ほど見た *早期リターン* のパターンを抽象化できるのです。

`try!` マクロの簡略化した定義はこうなります

```rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(err),
    });
}
```

（本当の定義はもっと洗練されています。 後ほど紹介します。）

`try!` マクロを使うと、最後の例をシンプルにすることが、とても簡単にできます。 場合分けと早期リターンを肩代わりしてくれますので、コードが締まって読みやすくなります。

- 独自のエラー型を定義する

標準ライブラリのいくつかのエラートレイトについて学ぶ前に、これまでの例にあったエラー型における `String` の使用を取り除くことで、この節を締めくくりたいと思います。

これまでの例では `String` を便利に使ってきました。 なぜなら、エラーは簡単に文字列へ変換できますし、問題が起こったその場で、文字列によるエラーを新たに作ることもできるからです。 しかし `String` を使ってエラーを表すことには欠点もあります。

ひとつ目の欠点は、エラーメッセージがコードのあちこちに散らかる傾向があることです。 エラーメッセージをどこか別の場所でまとめて定義することもできますが、特別に訓練された人でない限りは、エラーメッセージをコードに埋め込むことへの誘惑に負けてしまうでしょう。 実際、私たちは [以前の例](https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/1.9/ja/book/error-handling.md#code-error-double-string) でも、その通りのことをしました。

ふたつ目の、もっと重大な欠点は、 `String` への変換で *情報が欠落する* ことです。 もし全てのエラーを文字列に変換してしまったら、呼び出し元に渡したエラーが、不透明になってしまいます。 呼び出し元が `String` のエラーに対してできる唯一妥当なことは、それをユーザーに表示することだけです。 文字列を解析して、どのタイプのエラーだったか判断するのは、もちろん強固なやり方とはいえません。 （この問題は、ライブラリの中の方が、他のアプリケーションのようなものよりも、間違いなく重大なものになるでしょう。）

例えば `io::Error` 型には [`io::ErrorKind`](https://doc.rust-lang.org/std/io/enum.ErrorKind.html) が埋め込まれます。 これは *構造化されたデータ* で、IO操作において何が失敗したのかを示します。 エラーによって違った対応を取りたいこともあるので、このことは重要です。 （例： あなたのアプリケーションでは `BrokenPipe` エラーは正規の手順を踏んだ終了を意味し、 `NotFound` エラーはエラーコードと共に異常終了して、ユーザーにエラーを表示することを意味するかもしれません。） `io::ErrorKind` なら、呼び出し元でエラーの種類を調査するために、場合分けが使えます。 これは `String` の中からエラーの詳細がなんだったのか探りだすことよりも、明らかに優れています。

ファイルから整数値を取り出す例で `String` をエラー型として用いた代わりに、独自のエラー型を定義し、 *構造化されたデータ* によってエラー内容を表すことができます。 呼び出し元が詳細を検査したいときに備え、大元のエラーについての情報を取りこぼさないよう、努力してみましょう。

*多くの可能性のうちの一つ* を表す理想的な方法は、 `enum` を使って独自の直和型を定義することです。 このケースでは、エラーは `io::Error` もしくは `num::ParseIntError` でした。 ここから思い浮かぶ自然な定義は：

```rust
use std::io;
use std::num;

// 全ての型は `Debug` を導出するべきでしょうから、ここでも `Debug` を導出します。
// これにより `CliError` 値について、人間が十分理解できる説明を得られます。
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}
```

コードの微調整はいとも簡単です。 エラーを文字列に変換する代わりに、エラーに対応する値コンストラクタを用いて `CliError` 型に変換すればいいのです：

```rust
# #[derive(Debug)]
# enum CliError { Io(::std::io::Error), Parse(::std::num::ParseIntError) }
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn opener<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path).map_err(CliError::Io));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(CliError::Io));
    let n: i32 = try!(contents.trim().parse().map_err(CliError::Parse));
    Ok(2 * n)
}

fn main() {
    match opener("foobar") {
        Ok(n) => println!("{}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
```

ここでの変更点は、（エラーを文字列に変換する） `map_err(|e| e.to_string())` を、`map_err(CliError::Io)` や `map_err(CliError::Parse)` へ切り替えたことです。 こうして *呼び出し元* が、ユーザーに対してどの程度の詳細を報告するか決められるようになりました。 `String` をエラー型として用いることは、事実上、呼び出し元からこうした選択肢を奪ってしまいます。 `CliError` のような独自の `enum` エラー型を用いることは、 *構造化されたデータ* によるエラーの説明だけでなく、これまでと同様の使いやすさをもたらします。

目安となる方法は独自のエラー型を定義することですが、 `String` エラー型も、いざというときに役立ちます。 特にアプリケーションを書いているときなどはそうです。 もしライブラリを書いているのなら、呼び出し元の選択肢を理由もなく奪わないために、独自のエラー型を定義することを強く推奨します。

---

- 標準ライブラリのトレイトによるエラー処理

標準ライブラリでは、エラーハンドリングに欠かせないトレイトが、2つ定義されています： [`std::error::Error`](				https://doc.rust-lang.org/std/error/trait.Error.html)と [`std::convert::From`](https://doc.rust-lang.org/std/convert/trait.From.html)です。 `Error` はエラーを総称的に説明することを目的に設計されているのに対し、 `From` トレイトはもっと汎用的な、2つの異なる型の間で値を変換する役割を担います。

- `Error` トレイト

`Error` トレイトは 標準ライブラリで定義されています

```rust
use std::fmt::{Debug, Display};

trait Error: Debug + Display {
  /// エラーの簡単な説明
  fn description(&self) -> &str;

  /// このエラーの一段下のレベルの原因（もしあれば）
  fn cause(&self) -> Option<&Error> { None }
}
```

このトレイトは極めて一般的です。 なぜなら、エラーを表す *全て* の型で実装されることを目的としているからです。 この後すぐ見るように、このことは合成可能なコードを書くのに間違いなく役立ちます。 それ以外にも、このトレイトでは最低でも以下のようなことができます：

- エラーの `Debug` 表現を取得する。
- エラーのユーザー向け `Display` 表現を取得する。
- エラーの簡単な説明を取得する（`description` メソッド経由）。
- エラーの因果関係のチェーンが提供されているなら、それを調べる（`cause` メソッド経由）。

最初の2つは `Error` が `Debug` と `Display` の実装を必要としていることに由来します。 残りの2つは `Error` が定義している2つのメソッドに由来します。 `Error` の強力さは、実際に全てのエラー型が `Error` を実装していることから来ています。 このことは、全てのエラーを1つの トレイトオブジェクトとして存在量化(existentially quantify) できることを意味します。 これは `Box<dyn Error>` または `&dyn Error` と書くことで表明できます。 実際に `cause` メソッドは `&Error` を返し、これ自体はトレイトオブジェクトです。 `Error` トレイトのトレイトオブジェクトとしての用例については、後ほど再び取りあげます。

`Error` トレイトの実装例を見せるには、いまはこのくらいで十分でしょう。 [前の節](https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/1.9/ja/book/error-handling.md#独自のエラー型を定義する) で定義したエラー型を使ってみましょう：

```rust
use std::io;
use std::num;

// 全ての型は `Debug` を導出するべきでしょうから、ここでも `Debug` を導出します。
// これにより `CliError` 値について、人間が十分理解できる説明を得られます。
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}
```

このエラー型は2種類のエラー、つまり、IOを扱っているときのエラー、または、文字列を数値に変換するときのエラーが起こる可能性を示しています。 `enum` 定義のヴァリアントを増やせば、エラーの種類をいくらでも表現できます。

`Error` を実装するのは実に単純な作業です。 大抵は明示的な場合分けの繰り返しになります。

```rust
use std::error;
use std::fmt;

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // 下層のエラーは両方ともすでに `Display` を実装しているので、
            // それらの実装に従います。
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl error::Error for CliError {
    fn description(&self) -> &str {
        // 下層のエラーは両方ともすでに `Error` を実装しているので、
        // それらの実装に従います。
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Parse(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            // 注意：これらは両方とも `err` を、その具象型（`&dyn io::Error` か
            // `dyn &num::ParseIntError` のいずれか）から、トレイトオブジェクト
            // `&dyn Error` へ暗黙的にキャストします。どちらのエラー型も `Error` を
            // 実装しているので、問題なく動きます。
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}
```

これは極めて典型的な `Error` の実装だということに留意してください。 このように、それぞれのエラー型にマッチさせて、`description` と `cause` のコントラクトを満たします。

- `From` トレイト

`std::convert::From` は 標準ライブラリで定義されています



```rust
trait From<T> {
    fn from(T) -> Self;
}
```

嬉しいくらい簡単でしょ？ `From` は、ある特定の `T` という型 *から* 、別の型へ変換するための汎用的な方法を提供するので大変便利です （この場合の「別の型」とは実装の主体、つまり `Self` です）。 `From` を支えているのは 標準ライブラリで提供される一連の実装です。

`From` がどのように動くか、いくつかの例を使って紹介しましょう：

```rust
let string: String = From::from("foo");
let bytes: Vec<u8> = From::from("foo");
let cow: ::std::borrow::Cow<str> = From::from("foo");
```

たしかに `From` が文字列を変換するのに便利なことはわかりました。 でもエラーについてはどうでしょうか？ 結論から言うと、これが重要な実装です：

```rust
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a>
```

この実装では、 `Error` を実装した *全て* の型は、トレイトオブジェクト `Box<dyn Error>` に変換できると言っています。 これは、驚きに値するものには見えないかもしれませんが、一般的なコンテキストで有用なのです。

さっき扱った2つのエラーを覚えてますか？ 具体的には `io::Error` と `num::ParseIntError` でした。 どちらも `Error` を実装していますので `From` で動きます。

```rust
use std::error::Error;
use std::fs;
use std::io;
use std::num;

// エラーの値にたどり着くまで、何段階かのステップが必要です。
let io_err: io::Error = io::Error::last_os_error();
let parse_err: num::ParseIntError = "not a number".parse::<i32>().unwrap_err();

// では、こちらで変換します。
let err1: Box<dyn Error> = From::from(io_err);
let err2: Box<dyn Error> = From::from(parse_err);
```

ここに気づくべき、本当に重要なパターンがあります。 `err1` と `err2` の両方ともが *同じ型* になっているのです。 なぜなら、それらが存在量化型、つまり、トレイトオブジェクトだからです。 特にそれらの背後の型は、コンパイラの知識から *消去されます* ので、 `err1` と `err2` が本当に同じに見えるのです。 さらに私たちは同じ関数呼び出し `From::from` を使って `err1` と `err2` をコンストラクトしました。 これは `From::from` が引数とリターン型の両方でオーバーロードされているからです。

このパターンは重要です。 なぜなら、私たちが前から抱えていた問題を解決するからです： 同じ関数を使って、エラーを同一の型に変換する、確かな方法を提供するからです。

いよいよ、私たちの旧友 `try!` マクロを再訪するときが訪れました。

---

- 本当の `try!` マクロ

`try!` の定義は、以前このように提示されました：

```rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(err),
    });
}
```

これは本当の定義ではありません。 本当の定義は 標準ライブラリの中にあります

```rust
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(::std::convert::From::from(err)),
    });
}
```

文面上はわずかですが、非常に大きな違いがあります： エラーの値は `From::from` を経て渡されるのです。 これにより `try!` マクロは、はるかに強力になります。 なぜなら、自動的な型変換をただで手に入れられるのですから。

強力になった `try!` マクロを手に入れたので、以前書いた、ファイルを読み込んで内容を整数値に変換するコードを見直してみましょう：

```rust
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn opener<P: AsRef<Path>>(file_path: P) -> Result<i32, String> {
    let mut file = try!(File::open(file_path).map_err(|e| e.to_string()));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(|e| e.to_string()));
    let n = try!(contents.trim().parse::<i32>().map_err(|e| e.to_string()));
    Ok(2 * n)
}
```

以前 `map_err` の呼び出しを取り除くことができると約束しました。 もちろんです。ここでしなければいけないのは `From` と共に動く型を一つ選ぶだけでよいのです。 前の節で見たように `From` の実装の一つは、どんなエラー型でも `Box<Error>` に変換できます：

```rust
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn opener<P: AsRef<Path>>(file_path: P) -> Result<i32, Box<dyn Error>> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n = try!(contents.trim().parse::<i32>());
    Ok(2 * n)
}
```

理想的なエラーハンドリングまで、あと一歩です。 私たちのコードには、エラーハンドリングを終えた後も、ごくわずかなオーバーヘッドしかありません。 これは `try!` マクロが同時に3つのことをカプセル化するからです：

1. 場合分け
2. 制御フロー
3. エラー型の変換

これら3つが一つになったとき、コンビネータ、 `unwrap` の呼び出し、場合分けなどの邪魔者を排除したコードが得られるのです。

あとひとつ、些細なことが残っています： `Box<dyn Error>` 型は 不透明なのです。 もし `Box<dyn Error>` を呼び出し元に返すと、呼び出し元では背後のエラー型が何であるかを、（簡単には）調べられません。 この状況は `String` を返すよりは明らかに改善されてます。 なぜなら、呼び出し元では `description`や `cause`といったメソッドを呼ぶこともできるからです。 しかし `Box<dyn Error>` が不透明であるという制限は残ります。 （注意：これは完全な真実ではありません。 なぜならRustでは実行時のリフレクションができるからです。 この方法が有効なシナリオもありますが、[このセクションで扱う範囲を超えています](https://crates.io/crates/error) ）

では、私たちの独自のエラー型 `CliError` に戻って、全てを一つにまとめ上げましょう。

- 独自のエラー型を合成する

前の節では `try!` マクロの本当の定義を確認し、それが `From::from` をエラーの値に対して呼ぶことで、自動的な型変換をする様子を見ました。 特にそこでは、エラーを `Box<Error>` に変換しました。 これはたしかに動きますが、呼び出し元にとって、型が不透明になってしまいました。

これを直すために、すでによく知っている改善方法である独自のエラー型を使いましょう。 もう一度、ファイルの内容を読み込んで整数値に変換するコードです：

```rust
use std::fs::File;
use std::io::{self, Read};
use std::num;
use std::path::Path;

// 全ての型は `Debug` を導出するべきでしょうから、ここでも `Debug` を導出します。
// これにより `CliError` 値について、人間が十分理解できる説明を得られます。
#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

fn opener_verbose<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path).map_err(CliError::Io));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents).map_err(CliError::Io));
    let n: i32 = try!(contents.trim().parse().map_err(CliError::Parse));
    Ok(2 * n)
}
```

`map_err` がまだあることに注目してください。 なぜって、 [`try!`](https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/1.9/ja/book/error-handling.md#code-try-def) と [`From`](https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/1.9/ja/book/error-handling.md#code-from-def) の定義を思い出してください。 ここでの問題は `io::Error` や `num::ParseIntError` といったエラー型を、私たち独自の `CliError` に変換できる `From` の実装が無いことです。 もちろん、これは簡単に直せます！ `CliError` を定義したわけですから、それに対して `From` を実装できます：

```rust
# #[derive(Debug)]
# enum CliError { Io(io::Error), Parse(num::ParseIntError) }
use std::io;
use std::num;

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}
```

これらの実装がしていることは、`From` に対して、どうやって他のエラー型を元に `CliError` を作るのか、教えてあげているだけです。 このケースでは、単に対応する値コンストラクタを呼ぶことで構築しています。 本当に *普通は* これくらい簡単にできてしまいます。

これでようやく `opener` を書き直せます：

```rust
# use std::io;
# use std::num;
# enum CliError { Io(::std::io::Error), Parse(::std::num::ParseIntError) }
# impl From<io::Error> for CliError {
#     fn from(err: io::Error) -> CliError { CliError::Io(err) }
# }
# impl From<num::ParseIntError> for CliError {
#     fn from(err: num::ParseIntError) -> CliError { CliError::Parse(err) }
# }

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn opener<P: AsRef<Path>>(file_path: P) -> Result<i32, CliError> {
    let mut file = try!(File::open(file_path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    let n: i32 = try!(contents.trim().parse());
    Ok(2 * n)
}
```

ここでしたのは `map_err` を取り除くことだけです。 それらは `try!` マクロがエラーの値に対して `From::from` を呼ぶので、もう不要になりました。 これで動くのは、起こりうる全てのエラー型に対して `From` の実装を提供したからです。

もし `opener` 関数を変更して、なにか他の操作、例えば、文字列を浮動小数点数に変換させたいと思ったら、エラー型のヴァリアントを追加するだけです：

```rust
use std::io;
use std::num;

enum CliError {
    Io(io::Error),
    ParseInt(num::ParseIntError),
    ParseFloat(num::ParseFloatError),
}
```

そして、新しい `From` 実装を追加します

```rust
# enum CliError {
#     Io(::std::io::Error),
#     ParseInt(num::ParseIntError),
#     ParseFloat(num::ParseFloatError),
# }

use std::num;

impl From<num::ParseFloatError> for CliError {
    fn from(err: num::ParseFloatError) -> CliError {
        CliError::ParseFloat(err)
    }
}
```
