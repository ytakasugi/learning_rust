### std::collections


---

#### std::collections::HashMap

- Description

  2次プロービングとSIMDルックアップで実装されたハッシュマップです。

  デフォルトでは、`HashMap`は`HashDoS`攻撃に対する耐性を備えたハッシュアルゴリズムを使用します。アルゴリズムはランダムにシードされ、プログラムをブロックすることなく、ホストが提供する高品質で安全なランダムソースからシードを生成するために、合理的なベストエフォートが行われます。このため、シードのランダム性は、シードが生成されたときのシステムの乱数生成器の出力品質に依存します。特に、システムのブート時など、システムのエントロピープールが異常に低下しているときに生成されたシードは、品質が低くなる可能性があります。

  デフォルトのハッシュアルゴリズムは現在`SipHash 1-3`ですが、将来的には変更される可能性があります。`SipHash 1-3`の性能は中程度のサイズのキーでは非常に競争力がありますが、整数などの小さなキーや長い文字列などの大きなキーでは他のハッシュアルゴリズムの方が優れていますが、これらのアルゴリズムでは`HashDoS`のような攻撃を防ぐことはできません。

  このハッシュアルゴリズムは、デフォルト、`with_hasher`、`with_capacity_and_hasher`の各メソッドを使って、`HashMap`ごとに置き換えることができます。`crates.io`には、多くの代替ハッシュアルゴリズムがあります。

  キーが`Eq`と`Hash`のトレイトを実装していることが必要ですが、これは`#[derive(PartialEq, Eq, Hash)]`を使うことでよく実現できます。これらを自分で実装する場合は、以下の特性が成り立つことが重要です。

```
  k1 == k2 -> hash(k1) == hash(k2)
```

  言い換えれば、2つのキーが等しい場合、それらのハッシュは等しくなければなりません。

  キーがマップの中にある間に、Hashトレイトによって決定されるキーのハッシュや、`Eq`トレイトによって決定されるキーの等値性が変化するような方法でキーが修正されることは、論理エラーです。これは通常、`Cell`、`RefCell`、グローバルステート、I/O、または安全でないコードによってのみ可能です。このような論理エラーから生じる動作は規定されていませんが、未定義の動作になることはありません。これには、パニック、不正な結果、アボート、メモリリーク、終了しないことなどが含まれます。

  ハッシュテーブルの実装は、`Google`の`SwissTable`を`Rust`に移植したものです。オリジナルの`C++`バージョンの`SwissTable`はこちらで見ることができ、この`CppCon`の講演ではアルゴリズムがどのように動作するかの概要が説明されています。

- Exmaple

```rust
  use std::collections::HashMap;
  
  // 型推論では，明示的な型シグネチャを省略することができます
  // この例では `HashMap<String, String>` となります
  let mut book_reviews = HashMap::new();
  
  // Review some books.
  book_reviews.insert(
      "Adventures of Huckleberry Finn".to_string(),
      "My favorite book.".to_string(),
  );
  book_reviews.insert(
      "Grimms' Fairy Tales".to_string(),
      "Masterpiece.".to_string(),
  );
  book_reviews.insert(
      "Pride and Prejudice".to_string(),
      "Very enjoyable.".to_string(),
  );
  book_reviews.insert(
      "The Adventures of Sherlock Holmes".to_string(),
      "Eye lyked it alot.".to_string(),
  );
  
  // 特定のものをチェックします。
  // コレクションが所有する値(String)を格納している場合でも、
  // 参照(&str)を使用して照会することができます。
  if !book_reviews.contains_key("Les Misérables") {
      println!("We've got {} reviews, but Les Misérables ain't one.",
               book_reviews.len());
  }
  
  // おっと、このレビューは誤字脱字が多いので削除しましょう。
  book_reviews.remove("The Adventures of Sherlock Holmes");
  
  // いくつかのキーに関連する値を調べる。
  let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
  for &book in &to_find {
      match book_reviews.get(book) {
          Some(review) => println!("{}: {}", book, review),
          None => println!("{} is unreviewed.", book)
      }
  }
  
  // キーの値を検索します（キーが見つからない場合はパニックになります）。
  println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);
  
  // Iterate over everything.
  for (book, review) in &book_reviews {
      println!("{}: \"{}\"", book, review);
  }
```

  HashMapには[`Entry API`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.entry)も実装されており、キーとその値の取得、設定、更新、削除をより複雑な方法で行うことができます。

```rust
  use std::collections::HashMap;
  
  // type inference lets us omit an explicit type signature (which
  // would be `HashMap<&str, u8>` in this example).
  let mut player_stats = HashMap::new();
  
  fn random_stat_buff() -> u8 {
      // could actually return some random value here - let's just return
      // some fixed value for now
      42
  }
  
  // insert a key only if it doesn't already exist
  player_stats.entry("health").or_insert(100);
  
  // insert a key using a function that provides a new value only if it
  // doesn't already exist
  player_stats.entry("defence").or_insert_with(random_stat_buff);
  
  // update a key, guarding against the key possibly not being set
  let stat = player_stats.entry("attack").or_insert(100);
  *stat += random_stat_buff();
```

  カスタムのキータイプで`HashMap`を使用する最も簡単な方法は、`Eq`と`Hash`を派生させることです。また、`PartialEq`を派生させる必要があります。

```rust
  use std::collections::HashMap;
  
  #[derive(Hash, Eq, PartialEq, Debug)]
  struct Viking {
      name: String,
      country: String,
  }
  
  impl Viking {
      /// Creates a new Viking.
      fn new(name: &str, country: &str) -> Viking {
          Viking { name: name.to_string(), country: country.to_string() }
      }
  }
  
  // Use a HashMap to store the vikings' health points.
  let mut vikings = HashMap::new();
  
  vikings.insert(Viking::new("Einar", "Norway"), 25);
  vikings.insert(Viking::new("Olaf", "Denmark"), 24);
  vikings.insert(Viking::new("Harald", "Iceland"), 12);
  
  // Use derived implementation to print the status of the vikings.
  for (viking, health) in &vikings {
      println!("{:?} has {} hp", viking, health);
  }
```

  要素の固定されたリストを持つ`HashMap`は、配列から初期化することができます。

```rust
  use std::collections::HashMap;
  
  let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
      .iter().cloned().collect();
  // use the values stored in map
```



---

#### std::collections::hash_map::HashMap::insert

- Description

  キーと値のペアをマップに挿入します。

  マップにこのキーが存在しなかった場合は、`None`が返されます。

  マップにこのキーが存在していた場合は、値が更新され、古い値が返されます。ただし、キーは更新されません。これは、同一でなくても`==`できる型の場合に重要です。詳しくは、モジュールレベルのドキュメントを[参照](https://doc.rust-lang.org/stable/std/collections/index.html#insert-and-complex-keys)してください。

- Example

```rust
  use std::collections::HashMap;
  
  let mut map = HashMap::new();
  assert_eq!(map.insert(37, "a"), None);
  assert_eq!(map.is_empty(), false);
  
  map.insert(37, "b");
  assert_eq!(map.insert(37, "c"), Some("b"));
  assert_eq!(map[&37], "c");
```



---

#### std::collections::hash_map::HashMap::get

- Description

  キーに対応する値への参照を返します。

  キーはマップのキータイプの借用形式であれば何でも構いませんが、借用形式の`Hash`と`Eq`はキータイプのものと一致しなければなりません。

- Example

```rust
  use std::collections::HashMap;
  
  let mut map = HashMap::new();
  map.insert(1, "a");
  assert_eq!(map.get(&1), Some(&"a"));
  assert_eq!(map.get(&2), None);
```

  

---

#### std::collections::hash_map::HashMap::remove

- Description

  マップからキーを削除し、そのキーが以前にマップにあった場合は、そのキーの値を返します。

  キーはマップのキータイプの借用形式であれば何でも構いませんが、借用形式の`Hash`と`Eq`はキータイプのものと一致しなければなりません。

- Example

```rust
  use std::collections::HashMap;
  
  let mut map = HashMap::new();
  map.insert(1, "a");
  assert_eq!(map.remove(&1), Some("a"));
  assert_eq!(map.remove(&1), None);
```



---

#### std::collections::hash_map::HashMap::contains_key

- Description

  マップに指定されたキーに対応する値が含まれている場合、`true`を返します。

  キーはマップのキータイプの借用形式であれば何でもかまいませんが、借用形式の`Hash`と`Eq`はキータイプのものと一致しなければなりません。

- Example

```rust
  use std::collections::HashMap;
  
  let mut map = HashMap::new();
  map.insert(1, "a");
  assert_eq!(map.contains_key(&1), true);
  assert_eq!(map.contains_key(&2), false);
```



---

#### std::collections::hash_map::HashMap::values

- Desciption

  任意の順序ですべての値にアクセスするイテレータ。イテレータの要素タイプは`&'a V`です。

- Example

```rust
  use std::collections::HashMap;
  
  let mut map = HashMap::new();
  map.insert("a", 1);
  map.insert("b", 2);
  map.insert("c", 3);
  
  for val in map.values() {
      println!("{}", val);
  }
```



---

#### std::collections::HaseSet

- Description

  値が`()`である`HashMap`として実装されたハッシュセットです。

  `HashMap`型と同様に、`HashSet`は要素が`Eq`と`Hash`のトレイトを実装する必要があります。これは`#[derive(PartialEq, Eq, Hash)]`を使用することで実現できます。これらを自分で実装する場合は、次の特性が成り立つことが重要です。

```
  k1 == k2 -> hash(k1) == hash(k2)
```

  言い換えれば、2つのキーが等しい場合、それらのハッシュは等しくなければなりません。

  あるアイテムがセットの中にある間に、Hash形質によって決定されるそのアイテムのハッシュや、`Eq`トレイトによって決定されるそのアイテムの等価性が変化するような方法でアイテムが修正されることは、論理エラーです。これは通常、`Cell`、`RefCell`、グローバルステート、I/O、または安全でないコードによってのみ可能です。このような論理エラーの結果として生じる動作は規定されていませんが、未定義の動作になることはありません。これには、パニック、不正な結果、アボート、メモリリーク、終了しないことなどが含まれる。

- Example

```rust
  use std::collections::HashSet;
  // Type inference lets us omit an explicit type signature (which
  // would be `HashSet<String>` in this example).
  let mut books = HashSet::new();
  
  // Add some books.
  books.insert("A Dance With Dragons".to_string());
  books.insert("To Kill a Mockingbird".to_string());
  books.insert("The Odyssey".to_string());
  books.insert("The Great Gatsby".to_string());
  
  // Check for a specific one.
  if !books.contains("The Winds of Winter") {
      println!("We have {} books, but The Winds of Winter ain't one.",
               books.len());
  }
  
  // Remove a book.
  books.remove("The Odyssey");
  
  // Iterate over everything.
  for book in &books {
      println!("{}", book);
  }
```

  `HashSet`をカスタムタイプで使用する最も簡単な方法は、`Eq`と`Hash`を派生させることです。`PartialEq`も派生させなければなりませんが、これは将来的には`Eq`によって暗示されることになるでしょう。

```rust
  use std::collections::HashSet;
  #[derive(Hash, Eq, PartialEq, Debug)]
  struct Viking {
      name: String,
      power: usize,
  }
  
  let mut vikings = HashSet::new();
  
  vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
  vikings.insert(Viking { name: "Einar".to_string(), power: 9 });
  vikings.insert(Viking { name: "Olaf".to_string(), power: 4 });
  vikings.insert(Viking { name: "Harald".to_string(), power: 8 });
  
  // Use derived implementation to print the vikings.
  for x in &vikings {
      println!("{:?}", x);
  }
```

  要素の固定リストを持つ`HashSet`は、配列から初期化することができます。

```rust
  use std::collections::HashSet;
  
  let viking_names: HashSet<&'static str> =
      [ "Einar", "Olaf", "Harald" ].iter().cloned().collect();
  // use the values stored in the set
```



---

#### std::collections::hash_set::HashSet::union

- Description

  結合を表す値、つまり`self`や`other`のすべての値を、重複することなくアクセスします。

- Example

```rust
  use std::collections::HashSet;
  let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
  let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
  
  // Print 1, 2, 3, 4 in arbitrary order.
  for x in a.union(&b) {
      println!("{}", x);
  }
  
  let union: HashSet<_> = a.union(&b).collect();
  assert_eq!(union, [1, 2, 3, 4].iter().collect());
```



---

#### std::collections::hash_set::HashSet::contains

- Description

  セットに値が含まれている場合、`true`を返します。

  値はセットの値の型の借用形式であれば何でもかまいませんが、借用形式の`Hash`と`Eq`は値の型のものと一致する必要があります。

- Example

```rust
  use std::collections::HashSet;
  
  let set: HashSet<_> = [1, 2, 3].iter().cloned().collect();
  assert_eq!(set.contains(&1), true);
  assert_eq!(set.contains(&4), false);
```



---

#### std::collections::hash_set::HashSet::intersection

- Description

  交点を表す値、つまり自他共にある値にアクセスする。

- Example

```rust
  use std::collections::HashSet;
  let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
  let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
  
  // Print 2, 3 in arbitrary order.
  for x in a.intersection(&b) {
      println!("{}", x);
  }
  
  let intersection: HashSet<_> = a.intersection(&b).collect();
  assert_eq!(intersection, [2, 3].iter().collect());
```



---

#### std::collections::HashSet::hash_set::difference

- Description

  差異を表す値、つまり`self`にあって`other`にない値にアクセスします。

- Example

```rust
  use std::collections::HashSet;
  let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
  let b: HashSet<_> = [4, 2, 3, 4].iter().cloned().collect();
  
  // Can be seen as `a - b`.
  for x in a.difference(&b) {
      println!("{}", x); // Print 1
  }
  
  let diff: HashSet<_> = a.difference(&b).collect();
  assert_eq!(diff, [1].iter().collect());
  
  // Note that difference is not symmetric,
  // and `b - a` means something else:
  let diff: HashSet<_> = b.difference(&a).collect();
  assert_eq!(diff, [4].iter().collect());
```

---

### std::collections::BTreeSet

- Description

  `B-Tree`をベースにしたセットです。

  このコレクションの性能上の利点と欠点についての詳しい説明は、[`BTreeMap`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html)のドキュメントを参照してください。

  セットの中にあるアイテムが、[`Ord`](https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html)トレイトによって決定される、他のアイテムに対するアイテムの順序が変更されるような方法で変更されることは、論理エラーです。これは通常、[`Cell`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html)、[`RefCell`](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html)、グローバルステート、`I/O`、または`unsafe`コードによってのみ可能です。このような論理エラーから生じる動作は規定されていませんが、未定義の動作になることはありません。これには、パニック、不正な結果、アボート、メモリリーク、終了しないことなどが含まれる。

- Example

```rust
  use std::collections::BTreeSet;
  
  // Type inference lets us omit an explicit type signature (which
  // would be `BTreeSet<&str>` in this example).
  let mut books = BTreeSet::new();
  
  // Add some books.
  books.insert("A Dance With Dragons");
  books.insert("To Kill a Mockingbird");
  books.insert("The Odyssey");
  books.insert("The Great Gatsby");
  
  // Check for a specific one.
  if !books.contains("The Winds of Winter") {
      println!("We have {} books, but The Winds of Winter ain't one.",
               books.len());
  }
  
  // Remove a book.
  books.remove("The Odyssey");
  
  // Iterate over everything.
  for book in &books {
      println!("{}", book);
  }
```
