ar

---
#### char::to_digit

- Description

  文字を指定された基数の数字に変換します。

  ここでいう「基数」は、「ベース」と呼ばれることもあります。2の基数は2進数を、10の基数は10進数を、16の基数は16進数を表し、いくつかの一般的な値を示します。任意の基数がサポートされています。

  「桁」は以下の文字のみと定義されています。

  - `0-9`
  - `a-z`
  - `A-Z`

- Errors

  `char`が指定された基数の数字を参照していない場合は`None`を返します。

- Panics

  36以上の基数が与えられるとパニックになります。

- Example

  Basic Usage:

```rust
  assert_eq!('1'.to_digit(10), Some(1));
  assert_eq!('f'.to_digit(16), Some(15));
```

  数字でないものを通過すると失敗します。

```rust
  assert_eq!('f'.to_digit(10), None);
  assert_eq!('z'.to_digit(16), None);
```



---

#### char::is_whitespace

- Description

  この文字が`White_Space`プロパティを持っていれば、`true`を返します。

  `White_Space`は、`Unicode Character Database` [`PropList.txt`()](https://www.unicode.org/Public/UCD/latest/ucd/PropList.txt)で指定されています。

- Example

```rust
  assert!(' '.is_whitespace());
  
  // a non-breaking space
  assert!('\u{A0}'.is_whitespace());
  
  assert!(!'越'.is_whitespace());
```



---

#### char::is_ascii::digit

- Description

  値がASCIIの10進数であるかどうかをチェックします。u+0030 '0' ...= u+0039 '9'.

- Example

```rust
  let uppercase_a = 'A';
  let uppercase_g = 'G';
  let a = 'a';
  let g = 'g';
  let zero = '0';
  let percent = '%';
  let space = ' ';
  let lf = '\n';
  let esc: char = 0x1b_u8.into();
  
  assert!(!uppercase_a.is_ascii_digit());
  assert!(!uppercase_g.is_ascii_digit());
  assert!(!a.is_ascii_digit());
  assert!(!g.is_ascii_digit());
  assert!(zero.is_ascii_digit());
  assert!(!percent.is_ascii_digit());
  assert!(!space.is_ascii_digit());
  assert!(!lf.is_ascii_digit());
  assert!(!esc.is_ascii_digit());
```

---

#### std::char_indices

- Description

  文字列スライスの文字列とその位置を表すイテレータを返します。

  文字列スライスは有効なUTF-8で構成されているので、文字列スライスを`char`で反復することができます。このメソッドは、これらの文字列とそのバイト位置の両方のイテレータを返します。

  このイテレータはタプルを生成します。位置が最初で、文字が2番目です。

- Example

```rust
  let word = "goodbye";
  
  let count = word.char_indices().count();
  assert_eq!(7, count);
  
  let mut char_indices = word.char_indices();
  
  assert_eq!(Some((0, 'g')), char_indices.next());
  assert_eq!(Some((1, 'o')), char_indices.next());
  assert_eq!(Some((2, 'o')), char_indices.next());
  assert_eq!(Some((3, 'd')), char_indices.next());
  assert_eq!(Some((4, 'b')), char_indices.next());
  assert_eq!(Some((5, 'y')), char_indices.next());
  assert_eq!(Some((6, 'e')), char_indices.next());
  
  assert_eq!(None, char_indices.next());
```
