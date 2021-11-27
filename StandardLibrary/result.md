### [std::result](https://doc.rust-lang.org/std/result/index.html)

- Description

  `Result`型によるエラー処理。

  `Result<T, E>`は、エラーを返したり伝播させたりするのに使われる型です。これは、成功を表し、値を含む`Ok(T)`と、エラーを表し、エラー値を含む`Err(E)`という変種を持つ列挙型です。

```rust
  enum Result<T, E> {
     Ok(T),
     Err(E),
  }
```

  関数は、エラーが予想され、回復可能な場合は常に`Result`を返します。`std`クレートでは、`Result`は`I/O`で最も顕著に使用されます。

  `Result`を返す単純な関数は、次のように定義して使用することができます。

```rust
  #[derive(Debug)]
  enum Version { Version1, Version2 }
  
  fn parse_version(header: &[u8]) -> Result<Version, &'static str> {
      match header.get(0) {
          None => Err("invalid header length"),
          Some(&1) => Ok(Version::Version1),
          Some(&2) => Ok(Version::Version2),
          Some(_) => Err("invalid version"),
      }
  }
  
  let version = parse_version(&[1, 2, 3, 4]);
  match version {
      Ok(v) => println!("working with version: {:?}", v),
      Err(e) => println!("error parsing header: {:?}", e),
  }
```

  `Result`のパターンマッチは、単純なケースでは明確でわかりやすいですが、`Result`には、より簡潔に扱うことができる便利なメソッドがいくつかあります。

```rust
  let good_result: Result<i32, i32> = Ok(10);
  let bad_result: Result<i32, i32> = Err(10);
  
  // `is_ok`と`is_err`メソッドは、その言葉通りの働きをします。
  assert!(good_result.is_ok() && !good_result.is_err());
  assert!(bad_result.is_err() && !bad_result.is_ok());
  
  // `map` は `Result` を消費して別のものを生成します。
  let good_result: Result<i32, i32> = good_result.map(|i| i + 1);
  let bad_result: Result<i32, i32> = bad_result.map(|i| i - 1);
  
  //  計算を続けるには `and_then` を使います。
  let good_result: Result<bool, i32> = good_result.and_then(|i| Ok(i == 11));
  
  // エラーの処理には `or_else` を使用します。
  let bad_result: Result<i32, i32> = bad_result.or_else(|i| Ok(i + 20));
  
  // 結果を取り込み、その内容を `unwrap` で返します。
  let final_awesome_result = good_result.unwrap();
```

  

---

#### std::result::Result::map

- Description

  `Result<T, E>`を`Result<U, E>`にマップします。

  この関数は、2つの関数の結果を合成するために使用することができます。

- Example

~~~rust
  let line = "1\n2\n3\n4\n";
  
  for num in line.lines() {
      match num.parse::<i32>().map(|i| i * 2) {
          Ok(n) => println!("{}", n),
          Err(..) => {}
      }
  }
~~~
---

#### std::result::Result::and_then

  - Description

    結果が`Ok`の場合は`op`を呼び出し、そうでない場合は`self`の`Err`値を返す

- Example

```rust
  fn sq(x: u32) -> Result<u32, u32> { Ok(x * x) }
  fn err(x: u32) -> Result<u32, u32> { Err(x) }
  
  assert_eq!(Ok(2).and_then(sq).and_then(sq), Ok(16));
  assert_eq!(Ok(2).and_then(sq).and_then(err), Err(4));
  assert_eq!(Ok(2).and_then(err).and_then(sq), Err(2));
  assert_eq!(Err(3).and_then(sq).and_then(sq), Err(3));
```

  


---

#### [std::result::Result::is_err](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.is_err)

  - Description

    結果が`Err`なら`true`を返す。
    
- Example

```rust
  let x: Result<i32, &str> = Ok(-3);
  assert_eq!(x.is_err(), false);
  
  let x: Result<i32, &str> = Err("Some error message");
  assert_eq!(x.is_err(), true);
```

  

---

#### std::result::Result::map_err

- Description

  含まれる`Err`値に関数を適用し、`Ok`値はそのままにすることで、`Result<T, E>`を`Result<T, F>`にマッピングします。

  この関数は、エラーを処理しながら成功した結果を渡すために使用することができます。

- Example

```rust
  fn stringify(x: u32) -> String { format!("error code: {}", x) }
  
  let x: Result<u32, u32> = Ok(2);
  assert_eq!(x.map_err(stringify), Ok(2));
  
  let x: Result<u32, u32> = Err(13);
  assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()))
```

---

#### std::result::Result::unwrap_or_else

  - Description

    標準ライブラリで`Result<T, E>`に定義されている。

    unwrap_or_elseを使用することで、panic!ではない独自のエラーを返すことでができる。

    `Result<T,E>`の結果が`Ok`であれば、`Ok`が包んでいる値を返却する(`unwrap`に似ている)

    値が`Err`値なら、このメソッドはクロージャ内でコードを呼び、クロージャに定義した引数として`unwrap_or_else`に渡す匿名関数である。



---

#### std::result::Result::ok

- Description

  `Result<T, E>`から`Option<T>`に変換します。

  `self`を`Option<T>`に変換し、`self`を消費し、エラーがあればそれを破棄します。

- Example

```rust
  let x: Result<u32, &str> = Ok(2);
  assert_eq!(x.ok(), Some(2));
  
  let x: Result<u32, &str> = Err("Nothing here");
  assert_eq!(x.ok(), None);
```
