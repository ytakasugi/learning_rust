### std::option::Option

- Description

  オプションの種類です。詳細は、モジュールレベルの[ドキュメント](https://doc.rust-lang.org/std/option/index.html)を参照してください。

---

#### [std::option::Option::as_ref](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.as_ref) 

- Description

  `&Option<T>`から`Option<&T>`に変換します。

- Example

  `Option<String>`を`Option<usize>`に変換し、オリジナルを保持します。`map`メソッドは`self`引数を値で受け取り、オリジナルを消費するので、このテクニックでは `as_ref`を使用して、まず`Option`をオリジナル内部の値への参照にします。
  
~~~rust
  let text: Option<String> = Some("Hello, world!".to_string());
  // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
  // then consume *that* with `map`, leaving `text` on the stack.
  let text_length: Option<usize> = text.as_ref().map(|s| s.len());
  println!("still can print text: {:?}", text);
~~~



---

#### [std::option::Option::map_or](https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.map_or)

- Description

  含まれる値がある場合はその値に関数を適用し、ない場合は指定されたデフォルト値を返します。

  関数呼び出しの結果を渡す場合は、遅延評価を行う[map_or_else](https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.map_or_else)を使用することをお勧めします。

- Example

```rust
  let x = Some("foo");
  assert_eq!(x.map_or(42, |v| v.len()), 3);
  
  let x: Option<&str> = None;
  assert_eq!(x.map_or(42, |v| v.len()), 42);
```




---

#### std::option::Option::ok_or

- Description

  `Option<T>`を`Result<T, E>`に変換し、`Some(v)`を`Ok(v)`に、`None`を`Err(err)`にマッピングします。

  `ok_or`に渡された引数は熱心に評価されます。関数呼び出しの結果を渡す場合は、遅延的に評価される`ok_or_else`を使うことが推奨されます。

- Example

```rust
  let x = Some("foo");
  assert_eq!(x.ok_or(0), Ok("foo"));
  
  let x: Option<&str> = None;
  assert_eq!(x.ok_or(0), Err(0));
```



---

#### std::option::Option::ok_or_else

- Description

  `Option<T>`を`Result<T, E>`に変換し、`Some(v)`を`Ok(v)`に、`None`を`Err(err())`にマッピングします。

- Example

```rust
  let x = Some("foo");
  assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
  
  let x: Option<&str> = None;
  assert_eq!(x.ok_or_else(|| 0), Err(0));
```



---

#### std::option::Option::take

- Description

  オプションの値を削除し、代わりに`None`を残す

- Example

~~~rust
  let mut x = Some(2);
  let y = x.take();
  assert_eq!(x, None);
  assert_eq!(y, Some(2));
  
  let mut x: Option<u32> = None;
  let y = x.take();
  assert_eq!(x, None);
  assert_eq!(y, None);
~~~




---

#### std::option::Option::unwrap_or_default

- Description

  含まれる`Some`値またはデフォルト値を返します。

  `self`引数を消費し、`Some`の場合は含まれる値を、`None`の場合はその型のデフォルト値を返します。

- Example

  文字列を整数に変換し、整数に変換できない文字列は0（整数のデフォルト値）に変換します。 `parse`は、文字列を`FromStr`を実装した他の型に変換し、エラー時には`None`を返します。

```rust
  let good_year_from_input = "1909";
  let bad_year_from_input = "190blarg";
  let good_year = good_year_from_input.parse().ok().unwrap_or_default();
  let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();
  
  assert_eq!(1909, good_year);
  assert_eq!(0, bad_year);
```




---

#### std::option::Option::is_some

- Description

  オプションが`Some`値の場合は`true`を返します。

- Example

```rust
  let x: Option<u32> = Some(2);
  assert_eq!(x.is_some(), true);
  
  let x: Option<u32> = None;
  assert_eq!(x.is_some(), false);
```



---

#### std::option::Option::and_then

- Description

  `Option<T>`が`None`の場合は`None`を、そうでない場合はラップされた値で`f`を呼び出し、その結果を返します。

  この操作をフラットマップと呼ぶ言語もある。

- Example

```rust
  fn sq(x: u32) -> Option<u32> { Some(x * x) }
  fn nope(_: u32) -> Option<u32> { None }
  
  assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
  assert_eq!(Some(2).and_then(sq).and_then(nope), None);
  assert_eq!(Some(2).and_then(nope).and_then(sq), None);
  assert_eq!(None.and_then(sq).and_then(sq), None);
```




---

#### std::option::Option::is_none

- Description

  オプションが`None`の場合は`true`を返します。

- Example

```rust
  let x: Option<u32> = Some(2);
  assert_eq!(x.is_none(), false);
  
  let x: Option<u32> = None;
  assert_eq!(x.is_none(), true);
```



---

#### std::option::Option::unwrap_or

- Description

  含まれる`Some`値または提供されたデフォルト値を返します。

  `unwrap_or`に渡された引数は、熱心に評価されます。関数呼び出しの結果を渡している場合は、緩やかに評価される`unwrap_or_else`を使用することをお勧めします。

- Example

```rust
  assert_eq!(Some("car").unwrap_or("bike"), "car");
  assert_eq!(None.unwrap_or("bike"), "bike");
```
