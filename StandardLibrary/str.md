### std::str

---

#### std::str::from_utf8

- Description

  バイトのスライスを文字列のスライスに変換します。

  文字列スライス(`&str`)はバイト(`[u8]`)でできており、バイトスライス(`&[u8]`)はバイトでできているので、この関数は両者を変換します。`from_utf8()`は、バイトが有効なUTF-8であることを確認してから変換を行います。

  バイトスライスが有効な UTF-8 であることが確実で、有効性チェックのオーバーヘッドを発生させたくない場合は、この関数の安全ではないバージョンの`[from_utf8_unchecked]`があります。

  `str`の代わりに`String`が必要な場合は、`String::from_utf8`を検討してください。

  `[u8; N]`をスタック割り当てでき、その`&[u8]`を取ることができるので、この関数はスタック割り当てされた文字列を持つ一つの方法です。以下の例のセクションに例があります。

---

### str

---

#### str::contains

  - Description

    与えられたパターンが、この文字列スライスのサブスライスにマッチした場合に真を返す。
    そうでない場合は false を返す。
    パターンには、`&str`、`char`、文字列のスライス、文字がマッチするかどうかを判定する関数やクロージャを指定することができる。

---

#### str::chars

- Description

  文字列スライスの[`char`](https://doc.rust-lang.org/stable/std/primitive.char.html)に対するイテレータを返します。

  文字列スライスは有効なUTF-8で構成されているので、文字列スライスを[`char`](https://doc.rust-lang.org/stable/std/primitive.char.html)ごとに反復することができます。このメソッドは、そのようなイテレータを返します。

  [`char`](https://doc.rust-lang.org/stable/std/primitive.char.html)は Unicode Scalar Value を表しており、「文字」の概念とは異なることを覚えておく必要があります。実際に必要なのは、書記素クラスタのイテレータかもしれません。この機能はRustの標準ライブラリでは提供されていないので、代わりにcrates.ioをチェックしてください。

- Example

```rust
  let word = "goodbye";
  
  let count = word.chars().count();
  assert_eq!(7, count);
  
  let mut chars = word.chars();
  
  assert_eq!(Some('g'), chars.next());
  assert_eq!(Some('o'), chars.next());
  assert_eq!(Some('o'), chars.next());
  assert_eq!(Some('d'), chars.next());
  assert_eq!(Some('b'), chars.next());
  assert_eq!(Some('y'), chars.next());
  assert_eq!(Some('e'), chars.next());
  
  assert_eq!(None, chars.next());
```



---

#### str::split_whitespace

- Description

  文字列スライスをホワイトスペースで分割します。

  返されるイテレータは、元の文字列スライスのサブスライスで、任意の量のホワイトスペースで区切られた文字列スライスを返します。

  空白」は、Unicode Derived Core Property White_Spaceの条件に従って定義されます。ASCIIホワイトスペースでのみ分割したい場合は、[`split_ascii_whitespace`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_ascii_whitespace)を使用してください。

- Example

  Basic Usage:

```rust
  let mut iter = "A few words".split_whitespace();
  
  assert_eq!(Some("A"), iter.next());
  assert_eq!(Some("few"), iter.next());
  assert_eq!(Some("words"), iter.next());
  
  assert_eq!(None, iter.next());
```

  あらゆる種類のホワイトスペースが考慮されます。

```rust
  let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
  assert_eq!(Some("Mary"), iter.next());
  assert_eq!(Some("had"), iter.next());
  assert_eq!(Some("a"), iter.next());
  assert_eq!(Some("little"), iter.next());
  assert_eq!(Some("lamb"), iter.next());
  
  assert_eq!(None, iter.next());
```




---

#### str::lines

  - Description

    各行の文字列を文字列スライスとして、イテレータを返す。

    行は、改行（\ n）または改行によるキャリッジリターン（\ r \ n）のいずれかで終了する。

    最終行終了はオプションである。最終行終了で終わる文字列は、最終行終了のない、そうでなければ同一の文字列と同じ行を返す。



---

#### str::to_lowercase

  - Description

    この文字列スライスの小文字に相当するものを、新しい [String] として返す。
    `Lowercase`は、Unicode Derived Core Property Lowercaseの条項に従って定義される。
    大文字小文字を変更すると複数の文字に展開されてしまう文字があるため、この関数はパラメータをそのまま変更するのではなく、[String]として返す。



---

#### str::parse

- Description

  この文字列スライスを別の型にパースします。

  `parse`は非常に一般的なので、型の推論に問題が生じることがあります。そのため、`parse`は「ターボフィッシュ」として親しまれている構文（`::<>`）を目にする数少ない機会となっています。これは、型推論アルゴリズムが、どの型にパースしようとしているのかを具体的に理解するのに役立ちます。

  `parse`は、`FromStr`トレイトを実装したあらゆる型を解析することができます。

- Errors

  この文字列スライスを目的の型にパースできない場合は`Err`を返します。

- Example

```rust
  let four: u32 = "4".parse().unwrap();
  
  assert_eq!(4, four);
```

  Using the ‘turbofish’ instead of annotating `four`:

```rust
  let four = "4".parse::<u32>();
  
  assert_eq!(Ok(4), four);
```

  Failing to parse:

```rust
  let nope = "j".parse::<u32>();
  
  assert!(nope.is_err());
```


---

#### str::find

- Description

  この文字列スライスのパターンにマッチする最初の文字のバイトインデックスを返します。

  パターンにマッチしない場合は`None`を返します。

  パターンは、`&str`、`char`、`chars`のスライス、または、文字がマッチするかどうかを判断する関数やクロージャです。

- Example

  Simple patterns:

```rust
  let s = "Löwe 老虎 Léopard Gepardi";
  
  assert_eq!(s.find('L'), Some(0));
  assert_eq!(s.find('é'), Some(14));
  assert_eq!(s.find("pard"), Some(17));
```

  More complex patterns using point-free style and closures:

```rust
  let s = "Löwe 老虎 Léopard";
  
  assert_eq!(s.find(char::is_whitespace), Some(5));
  assert_eq!(s.find(char::is_lowercase), Some(1));
  assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
  assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

  Not finding the pattern:

```rust
  let s = "Löwe 老虎 Léopard";
  let x: &[_] = &['1', '2'];
  
  assert_eq!(s.find(x), None);
```
