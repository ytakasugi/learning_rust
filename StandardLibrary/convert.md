td::covert

- Description

  型間の変換のためのトレイト。

  このモジュールのトレイトは、ある型から別の型への変換方法を提供します。それぞれのトレイトは異なる目的を果たします。

  - 参照から参照への変換を安価に行うための`AsRef`トレイトを実装する。
  - `AsMut`トレイトを実装して、MutableからMutableへの変換を安価に行う。
    値から値への変換を行う`From`を実装する。
  - 現在のクレートの外にある型への値から値への変換を消費するために`Into` トレイトを実装する。
  - `TryFrom`と`TryInto`のトレイトは`From`と`Into`のように動作しますが、変換が失敗する可能性がある場合に実装する必要があります。

このモジュールのトレイトは、複数の型の引数をサポートするような汎用関数のトレイト境界としてよく使われます。例については、各トレイトのドキュメントを参照してください。

  ライブラリ作者としては、`Into<U>`や`TryInto<U>`よりも`From<T>`や`TryFrom<T>`の実装を常に好むべきです。`From`や`TryFrom`はより柔軟性が高く、標準ライブラリに包括的に実装されているおかげで、同等の`Into`や`TryInto`の実装が無料で提供されているからです。Rust 1.41 より前のバージョンをターゲットにしている場合、現在のクレートの外にある型に変換するときに、`Into`または`TryInto`を直接実装する必要があるかもしれません。

---

#### std::convert::Into

- Description

  入力値を消費する値から値への変換のこと。`From`の反対です。

  `Into`の実装を避け、代わりに`From`を実装すべきである。`From`を実装すると、標準ライブラリのブランケット実装のおかげで、自動的に`Into`の実装が提供されます。

  ジェネリック関数に`trait boundary`を指定する場合、`Into`のみを実装した型も使用できるようにするため、`From`よりも`Into`の使用を推奨する。

  注意：このトレイトは失敗してはいけません。変換に失敗する可能性がある場合は、`TryInto`を使用してください。

- 汎用的な実装

  - `From<T> for U`は`Into<U> for T`を意味します。
  - `Into`は反射的であり、`Into<T> for T`が実装されていることを意味します。

- 古いバージョンのRustでの外部型への変換のためのIntoの実装

  Rust 1.41以前では、目的の型が現在のクレートに含まれていない場合、`From`を直接実装することはできませんでした。例えば、このコードを見てみましょう。

```rust
  struct Wrapper<T>(Vec<T>);
  impl<T> From<Wrapper<T>> for Vec<T> {
      fn from(w: Wrapper<T>) -> Vec<T> {
          w.0
      }
  }
```

  これは古いバージョンの言語ではコンパイルできません。これを回避するために、Intoを直接実装することができます。

```rust
  struct Wrapper<T>(Vec<T>);
  impl<T> Into<Vec<T>> for Wrapper<T> {
      fn into(self) -> Vec<T> {
          self.0
      }
  }
```

  重要なのは、`Into`は`From`の実装を提供していないということです（`From`が`Into`を実装するように）。したがって、常に`From`の実装を試み、`From`が実装できない場合には`Into`にフォールバックする必要があります。

- Example

  `String`は`Into<Vec<u8>>`を実装しています。

  指定された型`T`に変換可能なすべての引数を取るジェネリック関数が欲しいことを表現するために，`Into`<T>の`trait bound`を使うことができます。例えば 関数`is_hello`は`Vec<u8>`に変換可能なすべての引数を取ります。

```rust
  fn is_hello<T: Into<Vec<u8>>>(s: T) {
     let bytes = b"hello".to_vec();
     assert_eq!(bytes, s.into());
  }
  
  let s = "hello".to_string();
  is_hello(s);
```


---

#### std::convert::TryInto

- Description

  selfを消費する変換の試みで、高価であるかどうかはわかりません。

  ライブラリの作者は、通常、このトレイトを直接実装すべきではなく、[`TryFrom`](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html)トレイトの実装を好むべきです。`TryFrom`トレイトは、より柔軟性があり、標準ライブラリの包括的な実装のおかげで、同等の`TryInto`の実装を無料で提供しています。これについての詳細は、[`Into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html)のドキュメントを参照してください。

- Implementing `TryInto`

  これは`Into`の実装と同じ制約と理由があります。詳細はこちらをご覧ください。


---

#### std::convert::From

- Descrition

  入力値を消費しながら値から値への変換を行う際に使用します。これは`Into`の逆数です。

  標準ライブラリのブランケット実装のおかげで、`From`を実装することで自動的にIntoの実装が提供されるため、常に`Into`よりも`From`を実装することを好むべきです。

  Rust 1.41以前のバージョンを対象とし、現在のクレート外の型に変換する場合のみ`Into`を実装してください。以前のバージョンでは、Rustの孤児化ルールのため、これらのタイプの変換を行うことができませんでした。詳細は`Into`を参照してください。

  一般的な関数でトレイト境界を指定する場合は、`From`よりも`Into`を使用することをお勧めします。この方法では、`Into`を直接実装した型も引数として使用できます。

  また、エラー処理を行う際にも`From`は非常に便利です。失敗する可能性のある関数を構築する場合、戻り値の型は一般的に `Result<T, E>`の形式になります。`From`は、関数が複数のエラー型をカプセル化した単一のエラー型を返すことを可能にするトレイトことで、エラー処理を単純化します。詳細については、「例」のセクションや書籍を参照してください。

  注意：このトレイトは失敗してはいけません。変換に失敗する可能性がある場合は、`TryFrom`を使用してください。

- Generic Implementation

  - `U`の`From<T>`は`T`の`Into<U>`を意味します。
  - `From`は反射的であり、`T`の`From<T>`が実装されていることを意味します。

- Example

  `String`は`From<&str>`を実装しています。

  `str`から`String`への明示的な変換は以下のように行われます。

~~~rust
  let string = "hello".to_string();
  let other_string = String::from("hello");
  
  assert_eq!(string, other_string);
~~~

  エラー処理を行う際に、独自のエラー型のために `From`を実装すると便利なことがよくあります。基礎となるエラー型を、基礎となるエラー型をカプセル化した独自のカスタムエラー型に変換することで、基礎となる原因に関する情報を失うことなく、単一のエラー型を返すことができます。演算子は、`From`を実装する際に自動的に提供される`Into<CliError>::into`を呼び出すことで、基礎となるエラー型を独自のエラー型に自動的に変換します。コンパイラは、`Into`のどの実装が使用されるべきかを推測します。

~~~rust
  use std::fs;
  use std::io;
  use std::num;
  
  enum CliError {
      IoError(io::Error),
      ParseError(num::ParseIntError),
  }
  
  impl From<io::Error> for CliError {
      fn from(error: io::Error) -> Self {
          CliError::IoError(error)
      }
  }
  
  impl From<num::ParseIntError> for CliError {
      fn from(error: num::ParseIntError) -> Self {
          CliError::ParseError(error)
      }
  }
  
  fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
      let mut contents = fs::read_to_string(&file_name)?;
      let num: i32 = contents.trim().parse()?;
      Ok(num)
  }
~~~




---

#### std::convert::TryFrom

- Description

  シンプルで安全な型変換ですが、ある状況下では制御された方法で失敗することがあります。これは`TryInto`の逆数です。

  これは、些細なことで成功するかもしれないが、特別な処理を必要とするかもしれない型変換を行っているときに便利です。例えば、、`From`トレイトを使って`i64`を`i32`に変換する方法はありません。なぜなら、`i64`には`i32`が表現できない値が含まれている可能性があり、変換によってデータが失われるからです。これには、`i64`を`i32`に切り詰める（基本的には`i64`の値を`i32::MAX`にモジュロして与える）か、単にi32::MAXを返すか、その他の方法で対処することになります。`From`トレイトは完全な変換を目的としているので、`TryFrom`トレイトは型変換がうまくいかない場合にプログラマに知らせ、それをどう処理するかを決めさせます。

- Generic Implementations

  - `U`の`TryFrom<T>`は`T`の`TryInto<U>`を意味する。
  - `try_from`は反射的で、`T`に対する`TryFrom<T>`が実装されていて、失敗しないことを意味しています -- `T`型の値に対して`T::try_from()`を呼び出した場合の関連するエラー型は`Infallible`です。この`!`型が安定したとき、`Infallible`と`!`は同等となるでしょう

  `TryFrom<T>`は以下のように実装できます。

```rust
  use std::convert::TryFrom;
  
  struct GreaterThanZero(i32);
  
  impl TryFrom<i32> for GreaterThanZero {
      type Error = &'static str;
  
      fn try_from(value: i32) -> Result<Self, Self::Error> {
          if value <= 0 {
              Err("GreaterThanZero only accepts value superior than zero!")
          } else {
              Ok(GreaterThanZero(value))
          }
      }
  }
```

- Example

  As described, [`i32`](https://doc.rust-lang.org/stable/std/primitive.i32.html) implements `TryFrom<`[`i64`](https://doc.rust-lang.org/stable/std/primitive.i64.html)`>`:

```rust
  use std::convert::TryFrom;
  
  let big_number = 1_000_000_000_000i64;
  // Silently truncates `big_number`, requires detecting
  // and handling the truncation after the fact.
  let smaller_number = big_number as i32;
  assert_eq!(smaller_number, -727379968);
  
  // Returns an error because `big_number` is too big to
  // fit in an `i32`.
  let try_smaller_number = i32::try_from(big_number);
  assert!(try_smaller_number.is_err());
  
  // Returns `Ok(3)`.
  let try_successful_smaller_number = i32::try_from(3);
  assert!(try_successful_smaller_number.is_ok());
```




---

#### std::convert::AsRef

  - Description

    簡単な参照間変換を行う。

    このトレイトは、可変参照間の変換に使用される`FnMut`に似ている。

    もし、高度な変換を行う必要がある場合は、`From`を`&T型`で実装するか、カスタム関数を実装するほうがよい。

    `AsRef`は、参照と同じシグネチャを持っていますが、いくつか異なる点がある。

     - `AsRef`とは異なり、参照は任意のTに対してブランケット実装(トレイト境界を満たすあらゆる型にトレイトを実装すること)を持っており、参照または値のどちらかを受け取るために使用できる

     - 参照では、参照した値の`Hash`、`Eq`、`Ord`が同等であることが要求される

     - 構造体の単一フィールドのみを借用したい場合は`Asref`を実施できますが、参照は実装できない。

    

    Note:このトレイトは失敗することができない。変換に失敗する可能性がある場合は、`Option<T>`または`Result<T, E>`を返す専用のメソッドを使用すること。

   - Generic Implementations

     `AsRef`は、内部の型が参照または変異可能な参照である場合に自動参照を行う (例: `foo.as_ref()`は、`foo`が`&mut Foo`または`&&mut Foo`の型を持っていても同じように動作する)。

  - Example

    トレイト境界を使うと、指定された型`T`に変換できる限り、異なる型の引数を受け入れることができます。

    例えば`AsRef<str>`を受け取るジェネリック関数を作ることで、`&str`に変換できるすべての参照を引数として受け入れたいことを表現しています。`String`と`&str`はどちらも`AsRef<str>`を実装しているので、どちらも入力引数として受け入れることができます。

~~~rust
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

let s = "hello";
is_hello(s);

let s = "hello".to_string();
is_hello(s);
~~~



---

#### std::convert::Infallible

- Description

  絶対に起こりえないエラーのためのエラータイプです。

  この列挙型にはバリアントがないため、この型の値が実際に存在することはありません。これは、結果が常に`Ok `であることを示すために、`Result`を使用してエラー タイプをパラメータ化する汎用 API に役立ちます。

  例えば、`TryFrom`トレイト（Resultを返す変換）には、逆の`Into`実装が存在するすべての型に対する包括的な実装があります。

```rust
  impl<T, U> TryFrom<U> for T where U: Into<T> {
      type Error = Infallible;
  
      fn try_from(value: U) -> Result<Self, Infallible> {
          Ok(U::into(value))  // Never returns `Err`
      }
  }
```
