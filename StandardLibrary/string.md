### std::string::String

---

#### String::from_utf8_lossy

- Description

  無効な文字を含むバイトのスライスを文字列に変換します。

  文字列はバイト([u8])でできており、バイトのスライス(&[u8])はバイトでできているので、この関数は両者を変換します。ただし、すべてのバイトスライスが有効な文字列であるわけではありません: 文字列は有効なUTF-8である必要があります。この変換の際、`from_utf8_lossy()`は無効な UTF-8 シーケンスを`U+FFFD REPLACEMENT CHARACTER`で置き換えます。

  バイトスライスが有効なUTF-8であることが確実で、変換のオーバーヘッドを発生させたくない場合は、この関数の安全でないバージョンである`from_utf8_unchecked`があります。

  この関数は`Cow<'a, str>`を返します。バイトスライスが無効なUTF-8であれば、置換文字を挿入する必要がありますが、これは文字列のサイズを変えることになるので、Stringが必要になります。しかし、すでに有効なUTF-8であれば、新しい割り当ては必要ありません。この戻り値型は、両方のケースを処理することができます。

---

#### std::string::String::as_str

- Description

  文字列全体を含む文字列スライスを抽出します。

- Example

```rust
  let s = String::from("foo");
  
  assert_eq!("foo", s.as_str());
```


---

#### std::string::String::into_bytes

- Description

  `String`をバイトベクターに変換します。

  これは、`String`を消費するので、その内容をコピーする必要はありません。

- Example

```rust
  let s = String::from("hello");
  let bytes = s.into_bytes();
  
  assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
```

