### [std::iter](https://doc.rust-lang.org/stable/std/iter/index.html)

- Description

  コンポーザブルな外部反復処理。

  ある種のコレクションを持っていて、そのコレクションの要素に対して操作を行う必要がある場合、すぐに 'イテレータ' に出くわすでしょう。イテレータはRustの慣用的なコードで頻繁に使用されているので、それらに慣れることは価値があります。

- Organizaiton

  このモジュールは主に型によって構成されています。

  - これらのトレイトは、どのような種類のイテレータが存在し、それを使って何ができるかを定義します。これらの特徴のメソッドは、特別に勉強する価値があります。
  - 関数は、いくつかの基本的なイテレータを作成するための便利な方法を提供しています。
  - 構造体は、このモジュールの特性にあるさまざまなメソッドの戻り値の型であることが多いです。通常、構造体自体ではなく、構造体を作成するメソッドを見たくなるでしょう。その理由についての詳細は、`Iterator`の実装を参照してください。



---

#### std::iter::FromIterator

  - Description

    型に対して`FromIterator`を実装することで、イテレータからどのように生成されるかを定義する。

    `FromIterator::from_iter()`が明示的にコールされることはほとんどなく、代わりに`Iterator::collect()`メソッドを使用する(詳細は、[`Iterator::collect()`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect)を参照)

---

#### std::iter::FromIterator::from_iter

- Description

  イテレータから値を作成します。

  詳細は、モジュールレベルの[ドキュメント](https://doc.rust-lang.org/std/iter/index.html)を参照してください。

- Example

```rust
  use std::iter::FromIterator;
  
  let five_fives = std::iter::repeat(5).take(5);
  
  let v = Vec::from_iter(five_fives);
  
  assert_eq!(v, vec![5, 5, 5, 5, 5]);
```




---

#### std::iter::repeat_with

- Description

  `A`型の要素を延々と繰り返す新しいイテレータを作成します。このイテレータは、提供されたクロージャであるリピーター、`F: FnMut() -> A`を適用することによって作成されます。

  `repeat_with()`関数は、リピーターを何度も何度も呼び出します。

  `repeat_with()`のような無限のイテレータは、有限にするために`Iterator::take() `のようなアダプタと一緒に使われることが多いです。

  必要なイテレータの要素タイプが`Clone`を実装していて、ソース要素をメモリ内に保持しても問題ない場合は、代わりに`repeat()`関数を使用するべきです。

  `repeat_with()`が生成するイテレータは[`DoubleEndedIterator`](https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html)ではありません。`repeat_with()`が`DoubleEndedIterator`を返すようにしたい場合は、GitHub の issue を開いてその使用例を説明してください。

- Example

  Basic usage:

```rust
  use std::iter;
  
  // let's assume we have some value of a type that is not `Clone`
  // or which don't want to have in memory just yet because it is expensive:
  #[derive(PartialEq, Debug)]
  struct Expensive;
  
  // a particular value forever:
  let mut things = iter::repeat_with(|| Expensive);
  
  assert_eq!(Some(Expensive), things.next());
  assert_eq!(Some(Expensive), things.next());
  assert_eq!(Some(Expensive), things.next());
  assert_eq!(Some(Expensive), things.next());
  assert_eq!(Some(Expensive), things.next());
```

  Using mutation and going finite:

```rust
  use std::iter;
  
  // From the zeroth to the third power of two:
  let mut curr = 1;
  let mut pow2 = iter::repeat_with(|| { let tmp = curr; curr *= 2; tmp })
                      .take(4);
  
  assert_eq!(Some(1), pow2.next());
  assert_eq!(Some(2), pow2.next());
  assert_eq!(Some(4), pow2.next());
  assert_eq!(Some(8), pow2.next());
  
  // ... and now we're done
  assert_eq!(None, pow2.next());
```



---

####  std::iter::Iterator::collect

  - Description

    イテレータをコレクションに変換する。

    `collect()`は、イテレータ可能なものなら何でも受け取り、関連するコレクションに変換することができる。これは標準ライブラリの中でも最も強力なメソッドのひとつで、さまざまなコンテキストで使用されている。

    `collect()`が使用される最も基本的なパターンは、あるコレクションを別のコレクションに変換すること。コレクションを取得し、それに対して`iter`を呼び出し、多くの変換を行い、最後に`collect()`を行う。

    `collect()`は、一般的なコレクションではない型のインスタンスを作成することもできる。例えば、文字列から`String`を作成したり、`Result<T, E>`アイテムのイテレータを`Result<Collection<T>, E>`に収集したりすることができる。詳細は[例](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect)を参照のこと。

    `collect()`は非常に一般的なので、型推論の問題を引き起こす可能性がある。そのため、collect() は「ターボフィッシュ」: `::<>`として親しまれている構文を目にすることができる数少ないもののひとつである。これは、推論アルゴリズムがどのコレクションにコレクションしようとしているのかを具体的に理解するのに役立つ。
    
- Example

```rust
  let a = [1, 2, 3];
  
  let doubled: Vec<i32> = a.iter()
                           .map(|&x| x * 2)
                           .collect();
  
  assert_eq!(vec![2, 4, 6], doubled);
```

  

---

#### std::iter::Iterator::filter_map

- Description

  フィルタリングとマップの両方を行うイテレータを作成します。

  返されるイテレータは、与えられたクロージャが`Some(value)``を返すような値だけを生成します。

  `filter_map`を使うと、`filter`と`map`の連鎖をより簡潔にすることができます。以下の例では、`map().filter().map()`を1回の`filter_map`の呼び出しに短縮することができます。

- Example

  Basic usage:

```rust
  let a = ["1", "two", "NaN", "four", "5"];
  
  let mut iter = a.iter().filter_map(|s| s.parse().ok());
  
  assert_eq!(iter.next(), Some(1));
  assert_eq!(iter.next(), Some(5));
  assert_eq!(iter.next(), None);
```

  以下は同じ例ですが、フィルターとマップを使用しています。

```rust
  let a = ["1", "two", "NaN", "four", "5"];
  let mut iter = a.iter().map(|s| s.parse()).filter(|s| s.is_ok()).map(|s| s.unwrap());
  assert_eq!(iter.next(), Some(1));
  assert_eq!(iter.next(), Some(5));
  assert_eq!(iter.next(), None);
```



---

#### std::iter::Iterator::partition

- Description

  イテレータを消費して、そこから2つのコレクションを作成します。

  `partition()`に渡される述語は、`true`または`false`を返すことができます。 `partition()`は、`true`を返したすべての要素と、`false`を返したすべての要素のペアを返します。

  [`is_partitioned()`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.is_partitioned)および[`partition_in_place()`](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.partition_in_place)も参照してください。

- Example

```rust
  let a = [1, 2, 3];
  
  let (even, odd): (Vec<i32>, Vec<i32>) = a
      .iter()
      .partition(|&n| n % 2 == 0);
  
  assert_eq!(even, vec![2]);
  assert_eq!(odd, vec![1, 3]);
```

  



---

#### std::iter::Iterator::any

- Description

  イテレータのいずれかの要素が述語にマッチするかどうかをテストします。

  `any()`は、`true`または`false`を返すクロージャを受け取ります。このクロージャをイテレータの各要素に適用し、それらのうちのどれかが true を返す場合は`any()`も同様です。すべてが `false`を返す場合は`false`を返します。

  つまり、他に何が起こっても結果は真であることを考えると、真を見つけるとすぐに処理を停止します。

  空のイテレータは`false`を返します。

---

#### std::iter::Iterator::find

  - Description

    述語を満たすイテレータの要素を検索します。

    `find()`は、`true`または`false`を返すクロージャを受け取ります。このクロージャをイテレータの各要素に適用し、どれか一つでも真を返せば、`find()`は`Some(element)`を返します。すべての要素が偽を返した場合は`None`を返します。

    `find()`は短絡的です。言い換えれば、クロージャが真を返すとすぐに処理を停止します。

    `find()`は参照を取り、多くのイテレータは参照を反復処理するので、引数が二重参照の場合には混乱する可能性があります。以下の例では、`&&x`でこの効果を見ることができます。
    
- Example

```rust
let a = [1, 2, 3];

assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));

assert_eq!(a.iter().find(|&&x| x == 5), None);
```

Stopping at the first `true`:

```rust
let a = [1, 2, 3];

let mut iter = a.iter();

assert_eq!(iter.find(|&&x| x == 2), Some(&2));

// we can still use `iter`, as there are more elements.
assert_eq!(iter.next(), Some(&3));
```

なお、`iter.find(f)`は`iter.filter(f).next()`と同等です。


---

#### std::iter::Iterator::find_map

  - Description

    イテレータの要素に関数を適用して、最初の`non-none`でない結果を返します。

    `iter.find_map(f)`は `iter.filter_map(f).next()`と同等です。

---

#### std::iter::Iterator::position

  - Description

    イテレータ内の要素を検索し、そのインデックスを返します。

    `position()`は、`true`または`false`を返すクロージャを受け取ります。このクロージャをイテレータの各要素に適用し、そのうちの 1 つが真を返す場合、 `position()`は`Some(index)`を返します。すべてが `false`を返す場合は `None`を返します。

    `position()`は短絡的な処理を行っています。

---

#### std::iter::Iterator::rposition

  - Description

    イテレータ内の要素を右から探し、そのインデックスを返します。

    `rposition()`は、`true`または`false`を返すクロージャを受け取ります。このクロージャをイテレータの各要素に、端から順に適用し、そのうちの1つが真を返すならば、`rposition()`は``Some(index)`を返します。すべてが`false`を返す場合は `None`を返します。

    `rposition()`は短絡的です。言い換えれば、真を見つけるとすぐに処理を停止します。

---

#### std::iter::Iterator::next

  - Description

    イテレータを進めて次の値を返す。反復が終了すると [None] を返す。個々のイテレータの実装は、反復処理を再開することを選択することができる。

---

#### std::iter::Iterator::take

- Description

  最初のn個の要素を生成するイテレータを作成します。

  - Example

    - Basic Usage

~~~rust
      let a = [1, 2, 3];
      
      let mut iter = a.iter().take(2);
      
      assert_eq!(iter.next(), Some(&1));
      assert_eq!(iter.next(), Some(&2));
      assert_eq!(iter.next(), None);
~~~

`take()`は、無限イテレータを使って有限にするためによく使われる

~~~ rust
      let mut iter = (0..).take(3);
      
      assert_eq!(iter.next(), Some(0));
      assert_eq!(iter.next(), Some(1));
      assert_eq!(iter.next(), Some(2));
      assert_eq!(iter.next(), None);
~~~

    - 利用可能な要素がn個よりも少ない場合、takeはそれ自身を基礎となるイテレータのサイズに制限します。

~~~rust
      let v = vec![1, 2];
      let mut iter = v.into_iter().take(5);
      assert_eq!(iter.next(), Some(1));
      assert_eq!(iter.next(), Some(2));
      assert_eq!(iter.next(), None);
~~~

---

#### std::iter::Iterator::skip

- Description

  最初の n 個の要素をスキップするイテレータを作成します。

  要素が消費された後、残りの要素が生成されます。このメソッドを直接オーバーライドするのではなく、代わりに `nth`メソッドをオーバーライドします。

  - Example

~~~rust
    let a = [1, 2, 3];
    
    let mut iter = a.iter().skip(2);
    
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
~~~



---

#### std::iter::Iterator::map

  - Description

    クロージャを受け取り、各要素上でそのクロージャを呼び出すイテレータを作成します。
    
    `map()`は、引数である`FnMut`を実装したものを使って、あるイテレータを別のイテレータに変換します。これは、元のイテレータの各要素に対してこのクロージャを呼び出す新しいイテレータを生成します。
    
    型で考えるのが得意な人は、`map()`をこのように考えることができます。ある型`A`の要素を与えるイテレータがあり、他の型`B`のイテレータが欲しい場合は`map()`を使用し、`A`を受け取り`B`を返すクロージャを渡すことができます。
    
    `map()`は、概念的には`for`ループに似ています。しかし、 map() は怠惰なので、すでに他のイテレータを使用している場合に使用するのが最適です。副次的な効果のために何らかのループを行う場合は、`map()`よりも`for`を使用した方が慣用的だと考えられています。

- Example

```rust
let a = [1, 2, 3];

let mut iter = a.iter().map(|x| 2 * x);

assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(4));
assert_eq!(iter.next(), Some(6));
assert_eq!(iter.next(), None);
```

ある種の副作用がある場合は、`map()`よりも`for`の方がいいでしょう。

```rust
// don't do this:
(0..5).map(|x| println!("{}", x));

// it won't even execute, as it is lazy. Rust will warn you about this.

// Instead, use for:
for x in 0..5 {
    println!("{}", x);
}
```

---

#### std::iter::Iterator::take_while

  - Description

    述語に基づいて要素を生成するイテレータを作成します。
    `take_while()`はクロージャを引数に取ります。これは、イテレータの各要素でこのクロージャを呼び出し、それが真を返す間に要素を生成します。
    `false`が返された後、`take_while()`の作業は終了し、残りの要素は無視されます。
    
  - Example

  ```rust
    let a = [-1i32, 0, 1];
    
    let mut iter = a.iter().take_while(|x| x.is_negative());
    
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), None);
  ```

    

---

#### std::iter::Iterator::filter

  - Description

    クロージャを使用して要素を生成するかどうかを決定するイテレータを作成します。
    要素が与えられると、クロージャは`true`または`false`を返さなければなりません。返されるイテレータは、クロージャが`true`を返す要素のみを返します。
    
- Example

```rust
  let a = [0i32, 1, 2];
  
  let mut iter = a.iter().filter(|x| x.is_positive());
  
  assert_eq!(iter.next(), Some(&1));
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), None);
```

  `filter()`に渡されたクロージャは参照を取り、多くのイテレータは参照を反復するので、クロージャの型が二重参照であるという混乱を招く可能性があります。
  
```rust
  let a = [0, 1, 2];
  
  let mut iter = a.iter().filter(|x| **x > 1); // need two *s!
  
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), None);
```
  
  その代わりに、引数にデストラクションを使用して 1 つを取り除くのが一般的です。
  
```rust
  let a = [0, 1, 2];
  
  let mut iter = a.iter().filter(|&x| *x > 1); // both & and *
  
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), None);
```
  
  または
  
```rust
  let a = [0, 1, 2];
  
  let mut iter = a.iter().filter(|&&x| x > 1); // two &s
  
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), None);
```
  
  なお、`iter.filter(f).next()`は`iter.find(f)`と同じです。

---

#### std::iter::Iterator::fold

  - Description

    関数を適用し、単一の最終値を生成するイテレータメソッド。
    `fold()`は、2つの引数を取ります: 初期値と、2つの引数を持つクロージャ: 'アキュムレータ' と要素です。クロージャは、アキュムレータが次の反復のために持つべき値を返します。
    初期値は、アキュムレータが最初の呼び出しで持つ値です。
    このクロージャをイテレータの各要素に適用した後、`fold()`はアキュムレータを返します。
    この操作は'reduce'や'inject'と呼ばれることもあります。
    折りたたみは、何かのコレクションを持っていて、そこから単一の値を生成したいときに便  利です。
    
    注意:`fold()`や、イテレータ全体を横断する同様のメソッドは、結果が有限時間内に決定可能な   トレイトあって   も、無限のイテレータでは終了しないことがあります。
    
- Example

```rust
  let a = [1, 2, 3];
  
  // the sum of all of the elements of the array
  let sum = a.iter().fold(0, |acc, x| acc + x);
  
  assert_eq!(sum, 6);
```

  ここでは、イテレーションの各ステップを説明します。

  | element | acc  | x    | result |
  | :------ | :--- | :--- | :----- |
  |         | 0    |      |        |
  | 1       | 0    | 1    | 1      |
  | 2       | 1    | 2    | 3      |
  | 3       | 3    | 3    | 6      |

  そして、最終的な結果、6となりました。

  この例では、`fold()`の左結合の性質を示しています。つまり、初期値から始まり、各要素の前から後ろまで、文字列を構築します。

```rust
  let numbers = [1, 2, 3, 4, 5];
  
  let zero = "0".to_string();
  
  let result = numbers.iter().fold(zero, |acc, &x| {
      format!("({} + {})", acc, x)
  });
  
  assert_eq!(result, "(((((0 + 1) + 2) + 3) + 4) + 5)");
```

  イテレータをあまり使ったことがない人は、結果を構築するためにリストを使ったforループを使うのが一般的です。それらは、`fold()`に変えることができます。

```rust
  let numbers = [1, 2, 3, 4, 5];
  
  let mut result = 0;
  
  // for loop:
  for i in &numbers {
      result = result + i;
  }
  
  // fold:
  let result2 = numbers.iter().fold(0, |acc, &x| acc + x);
  
  // they're the same
  assert_eq!(result, result2);
```

  

---

#### std::iter::Iterator::enumerate

- Description

  現在の反復回数と次の値を与えるイテレータを作成します。

  返されるイテレータは、ペア`(i, val)`を返します。

  `enumerate()`は、そのカウントを`usize`として保持します。異なるサイズの整数でカウントしたい場合は、`zip`関数も同様の機能を提供します。
  
- Over Behavior

  このメソッドはオーバーフローに対するガードをしていないので、`usize::MAX`を超える要素を列挙すると、間違った結果になるか、パニックになります。デバッグアサーションが有効な場合は、パニックが保証されます。

- Panics

  返されたイテレータは、返されるべきインデックスが`usize`をオーバーフローするとパニックになるかもしれません。

- Example

```rust
  let a = ['a', 'b', 'c'];
  
  let mut iter = a.iter().enumerate();
  
  assert_eq!(iter.next(), Some((0, &'a')));
  assert_eq!(iter.next(), Some((1, &'b')));
  assert_eq!(iter.next(), Some((2, &'c')));
  assert_eq!(iter.next(), None);
```

  

---

#### std::iter::Iterator::all

- Description

  イテレータのすべての要素が、ある述語にマッチするかどうかをテストします。

  `all()`は、`true`または`false`を返すクロージャを受け取ります。このクロージャをイテレータの各要素に適用し、それらがすべて真を返した場合は、`all()`も真を返します。どれか一つでも偽を返せば、偽を返します。

  `all()`は短絡的です。言い換えれば、偽を見つけたらすぐに処理を停止し、他に何が起こっても結果は偽になるということです。

  空のイテレータはtrueを返します。

- Example

```rust
  let a = [1, 2, 3];
  
  assert!(a.iter().all(|&x| x > 0));
  
  assert!(!a.iter().all(|&x| x > 2));
```

  Stopping at the first `false`:

```rust
  let a = [1, 2, 3];
  
  let mut iter = a.iter();
  
  assert!(!iter.all(|&x| x != 2));
  
  // we can still use `iter`, as there are more elements.
  assert_eq!(iter.next(), Some(&3));
```

  

---

#### std::iter::Iterator::peekable

- Description

  `peek`を使ってイテレータの次の要素を消費することなく見ることができるイテレータを作成します。

  イテレータに`peek`メソッドを追加します。詳細は、そのドキュメントを[参照](https://doc.rust-lang.org/stable/std/iter/struct.Peekable.html#method.peek)してください。

  `peek`が初めて呼ばれたとき、基礎となるイテレータはまだ進んでいることに注意してください。次の要素を取得するために、基礎となるイテレータで next が呼び出されるため、`next`メソッドの副作用 (次の値を取得する以外のこと) が発生します。

- Example

```rust
  let xs = [1, 2, 3];
  
  let mut iter = xs.iter().peekable();
  
  // peek() lets us see into the future
  assert_eq!(iter.peek(), Some(&&1));
  assert_eq!(iter.next(), Some(&1));
  
  assert_eq!(iter.next(), Some(&2));
  
  // we can peek() multiple times, the iterator won't advance
  assert_eq!(iter.peek(), Some(&&3));
  assert_eq!(iter.peek(), Some(&&3));
  
  assert_eq!(iter.next(), Some(&3));
  
  // after the iterator is finished, so is peek()
  assert_eq!(iter.peek(), None);
  assert_eq!(iter.next(), None);
```



---

#### std::iter::Iterator::peek

- Description

  イテレータを進めずに`next()`の値への参照を返します。

  `next`同様、値がある場合は`Some(T)`で包まれます。しかし、イテレーションが終わった場合はNoneが返されます。

  `peek()`は参照を返し、多くのイテレータは参照を反復しているので、戻り値が二重参照になってしまうという紛らわしい状況になることがあります。以下の例では、この影響を見ることができます。

- Example

```rust
  let xs = [1, 2, 3];
  
  let mut iter = xs.iter().peekable();
  
  // peek() lets us see into the future
  assert_eq!(iter.peek(), Some(&&1));
  assert_eq!(iter.next(), Some(&1));
  
  assert_eq!(iter.next(), Some(&2));
  
  // The iterator does not advance even if we `peek` multiple times
  assert_eq!(iter.peek(), Some(&&3));
  assert_eq!(iter.peek(), Some(&&3));
  
  assert_eq!(iter.next(), Some(&3));
  
  // After the iterator is finished, so is `peek()`
  assert_eq!(iter.peek(), None);
  assert_eq!(iter.next(), None);
```



---

#### std::iter::Iterator::rev

- Description

  イテレータの方向を反転させます。

  通常、イテレータは左から右に向かって反復します。`rev()`を使うと、イテレータは右から左に向かって反復されます。

  これはイテレータに終端がある場合にのみ可能なので、`rev()`は [DoubleEndedIterator](https://doc.rust-lang.org/stable/std/iter/trait.DoubleEndedIterator.html)でのみ動作します。

- Example

```rust
  let a = [1, 2, 3];
  
  let mut iter = a.iter().rev();
  
  assert_eq!(iter.next(), Some(&3));
  assert_eq!(iter.next(), Some(&2));
  assert_eq!(iter.next(), Some(&1));
  
  assert_eq!(iter.next(), None);
```




---

#### std::iter::Iterator::nth

  - Description

    イテレータの n 番目の要素を返す。

    ほとんどのインデックス操作と同様に、カウントはゼロから始まるので、 `nth(0)`は最初の値を返し、 `nth(1)`は 2 番目の値を返す。

    返された要素と同様に、先行するすべての要素がイテレータから消費されることに注意すること。つまり、先行する要素は破棄され、同じイテレータで `nth(0)`を複数回呼び出すと、異なる要素が返されることになる。

    `nth()`は、n がイテレータの長さ以上であれば `None`を返す。

    

---

#### std::iter::IntoIterator

- Description

  イテレータへの変換。

  型に対して`IntoIterator`を実装することで、それをイテレータに変換する方法を定義します。これは、ある種のコレクションを記述する型ではよくあることです。

  `IntoIterator`を実装する利点のひとつは、あなたの型が`Rust`の`for loop`構文で動作することです。

---

#### std::iter::Iterator::cloned

- Description

  すべての要素のクローンを作成するイテレータを作成します。

  これは、`&T`に対するイテレータを持っていて、`T`に対するイテレータが必要な場合に便利です。

- Example

```rust
  let a = [1, 2, 3];
  
  let v_cloned: Vec<_> = a.iter().cloned().collect();
  
  // cloned is the same as .map(|&x| x), for integers
  let v_map: Vec<_> = a.iter().map(|&x| x).collect();
  
  assert_eq!(v_cloned, vec![1, 2, 3]);
  assert_eq!(v_map, vec![1, 2, 3]);
```




---

#### std::iter::Iterator::zip

- Description

  2 つのイテレータを、ペアの単一のイテレータに「Zip Up」します。

  `zip()`は、他の2つのイテレータを反復処理し、最初の要素が最初のイテレータから、2番目の要素が2番目のイテレータから得られるタプルを返す新しいイテレータを返します。

  言い換えれば、2つのイテレータをまとめて1つのイテレータにします。

  どちらかのイテレータが`None`を返した場合、圧縮されたイテレータの次の要素は`None`を返します。最初のイテレータが`None`を返した場合、`zip`は短絡的に実行され、2 番目のイテレータの`next`は呼び出されません。

- Example

```rust
  let a1 = [1, 2, 3];
  let a2 = [4, 5, 6];
  
  let mut iter = a1.iter().zip(a2.iter());
  
  assert_eq!(iter.next(), Some((&1, &4)));
  assert_eq!(iter.next(), Some((&2, &5)));
  assert_eq!(iter.next(), Some((&3, &6)));
  assert_eq!(iter.next(), None);
```


---

#### std::iter::Iterator::for_each

- Description

  イテレータの各要素でクロージャを呼び出します。

  これはイテレータに`for`ループを使うのと同じですが、クロージャから`break`や`continue`はできません。一般的には`for`ループを使った方がわかりやすいですが、長いイテレータチェーンの最後にあるアイテムを処理する場合は`for_each`の方が読みやすいかもしれません。場合によっては`for_each`の方がループよりも速いかもしれません。なぜなら、`Chain`のようなアダプタでは内部で反復処理を行うからです。

- Example

```rust
  use std::sync::mpsc::channel;
  
  let (tx, rx) = channel();
  (0..5).map(|x| x * 2 + 1)
        .for_each(move |x| tx.send(x).unwrap());
  
  let v: Vec<_> =  rx.iter().collect();
  assert_eq!(v, vec![1, 3, 5, 7, 9]);
```

  このような小さな例では、`for`ループの方がすっきりするかもしれませんが、イテレータを長くして機能的なスタイルを維持するには`for_each`の方が望ましいかもしれません。

```rust
  (0..5).flat_map(|x| x * 100 .. x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{}:{}", i, x));
```