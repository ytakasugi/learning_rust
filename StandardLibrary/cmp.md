### std::cmp

---

#### std::cmp::Eq

- Description

  std::cmp::Eq
  等値関係である等値比較の特徴。

  これは、`a == b`と`a != b`が厳密な逆数であることに加えて、(すべての `a`, `b`,`c`に対して) 等価でなければならないことを意味する。

  - reflexive:`a == a`
  - symmetric:`a == b`は`b == a`を意味する; そして
  - transitive:`a == b`と`b == c`は`a ==c`を意味します。

  このプロパティはコンパイラではチェックできないので、`Eq`は`PartialEq`を意味し、余分なメソッドはありません。

- Derivable

  このトレイトは、`#[derive]`と一緒に使うことができます。`derive`の場合、`Eq`には余分なメソッドがないので、これは部分的な等価関係ではなく、等価関係であることをコンパイラに知らせているだけです。`derive`はすべてのフィールドが`Eq`であることを必要としますが、これは必ずしも望ましいことではありません。

- Example

  導出ストラテジーを使用できない場合は、メソッドを持たない Eq を実装していることを指定します。

~~~rust
  enum BookFormat { Paperback, Hardback, Ebook }
  struct Book {
      isbn: i32,
      format: BookFormat,
  }
  impl PartialEq for Book {
      fn eq(&self, other: &Self) -> bool {
          self.isbn == other.isbn
      }
  }
  　impl Eq for Book {}
~~~



---

#### std::cmp::Ord

- Description

  合計順序を形成する型のトレイト。
  次数は，(すべての`a`, `b`, `c`に対して)次数であれば合計次数です．

  合計で非対称： `a < b, `a == b`, `a > b`のうち、いずれか1つが真である。
  推移的な場合、`a < b`と`b < c`は`a < c`を意味します。

  - Lexicographical comparison
    辞書的比較は，次のようなトレイトを持つ操作です．
    - 2つのシーケンスが要素ごとに比較されます．
    - 最初のミスマッチ要素は，どちらのシーケンスが他のシーケンスよりも辞書的に小さいか大きいかを定義します．
    - 一方のシーケンスが他方のシーケンスの接頭辞である場合，短い方のシーケンスは他方のシーケンスよりも辞書的に小さいです．
    - 2つのシーケンスが等価な要素を持ち、同じ長さである場合、そのシーケンスは辞書的に等しくなります。
    - 空のシーケンスは，空でないシーケンスよりもレキシコグラフ的に小さくなります．
    - 2つの空の配列は辞書的に等しい。

  - `Ord`を実装するにはどうすればよいか
    `Ord`は型が`PartialOrd`と`Eq`（`PartialEq`を必要とします）であることが必要です。
    次に，`cmp`の実装を定義しなければなりません．型のフィールドで`cmp`を使用すると便利です。
    `PartialEq`、`PartialOrd`、および`Ord`の実装は、互いに一致している必要があります。つまり`a.cmp（b）== Ordering :: Equal`は、すべての`a`と`b`について`a == b`および`Some（a.cmp（b））== a.partial_cmp（b）`である場合に限ります。
    ここでは、`id`と名前を無視して身長だけでソートしたい場合の例を示します。

~~~rust
  use std::cmp::Ordering;
  
  #[derive(Eq)]
  struct Person {
      id: u32,
      name: String,
      height: u32,
  }
  
  impl Ord for Person {
      fn cmp(&self, other: &Self) -> Ordering {
          self.height.cmp(&other.height)
      }
  }
  
  impl PartialOrd for Person {
      fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
          Some(self.cmp(other))
      }
  }
  
  impl PartialEq for Person {
      fn eq(&self, other: &Self) -> bool {
          self.height == other.height
      }
  }
~~~



---

#### std::cmp::Ord::max

- Description

  2 つの値の最大値を比較して返します。

  比較の結果、2つの値が等しいと判断された場合は、2番目の引数を返します。

- Example

```rust
  assert_eq!(2, 1.max(2));
  assert_eq!(2, 2.max(2));
```

---

#### std::cmp::Ord::min

- Description

  2 つの値の最小値を比較して返します。

  比較の結果、2 つの値が等しいと判断された場合は、最初の引数を返します。

- Example

```rust
  assert_eq!(1, 1.min(2));
  assert_eq!(2, 2.min(2));
```



---

#### std::cmp::Ord::cmp

- Description

  このメソッドは、`self`と`other`の間の順序を返します。

  慣習的に、`self.cmp(&other)`は、真の場合、式`self <operator> other`に一致する順序を返します。

- Example

```rust
  use std::cmp::Ordering;
  
  assert_eq!(5.cmp(&10), Ordering::Less);
  assert_eq!(10.cmp(&5), Ordering::Greater);
  assert_eq!(5.cmp(&5), Ordering::Equal);
```



---

#### std::cmp::Ordering::reverse

- Description

  順番を逆にします。

  - 昇順は降順に
  - 降順は昇順に
  - 等しいものは等しいままに

- Example

```rust
  use std::cmp::Ordering;
  
  assert_eq!(Ordering::Less.reverse(), Ordering::Greater);
  assert_eq!(Ordering::Equal.reverse(), Ordering::Equal);
  assert_eq!(Ordering::Greater.reverse(), Ordering::Less);
```

  この方法では、比較対象を逆にすることができます。

```rust
  let data: &mut [_] = &mut [2, 10, 5, 8];
  
  // sort the array from largest to smallest.
  data.sort_by(|a, b| a.cmp(b).reverse());
  
  let b: &mut [_] = &mut [10, 8, 5, 2];
  assert!(data == b);
```

  

---

#### std::cmp::PartialEq

  - Description

    半同値関係にある等式の比較を行うトレイト。

    このトレイトは、完全な等価関係を持たない型に対して、部分的な等価関係を可能にします。

    例えば、浮動小数点では`Nan! = Nan`なのでPartialEqを実装していますが、Eqは実装していない。

    形式的には、(すべてのa,b,cに対して)等価でなければならない。

    対称的：`a == b`は`b == a`を意味する。
    推移的: `a == b`と`b == c`は`a == c`を意味する。
    これらの要件は、トレイト自体が対称的かつ推移的に実装されなければならないことを意味していることに注意すること。

    もし`T: PartialEq<U>`と `U: PartialEq<V>`の場合、 `U: PartialEq<T>`と`T: PartialEq<V>`となる。

    このトレイトは、#[derive]と一緒に使うことができる。構造体で導出された場合、すべてのフィールドが等しい場合は2つのインスタンスが等しく、いずれかのフィールドが等しくない場合は等しくない。enumsで導出された場合、各バリアントはそれ自身と等しく、他のバリアントとは等しくない。

    `PartialEq`は`eq`メソッドを実装する必要があるだけで、`PartialEq`は`eq`メソッドを使用して定義されている。

    を手動で実装する場合は、`eq`は`PartialEq `の厳密な逆数であるというルールを守らなければならない。

    つまり、`a != b`の場合に限り`!(a==b)`

    `PartialEq`、`PartialOrd`、`Ord`の実装は互いに一致していなけならない。いくつかのトレイトを導出し、他のトレイトを手動で実装することで、誤ってこれらのトレイトを一致させてしまうことは簡単である。

  - Example

    フォーマットが異なっていても、ISBN が一致していれば 2 冊の本が同じ本とみなされるドメインの実装例。

~~~rust
    enum BookFormat {
        Paperback,
        Hardback,
        Ebook,
    }
    
    struct Book {
        isbn: i32,
        format: BookFormat,
    }
    
    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }
    
    let b1 = Book { isbn: 3, format: BookFormat::Paperback };
    let b2 = Book { isbn: 3, format: BookFormat::Ebook };
    let b3 = Book { isbn: 10, format: BookFormat::Paperback };
    
    assert!(b1 == b2);
    assert!(b1 != b3);
~~~

---

#### std::cmp::PartialOrd

  - Description
    並べ替え順序で比較できる値の特徴。
    比較は，すべての`a`, `b`, `c`について，次の条件を満たさなければならない．
    非対称性: `a < b`の場合は`!（a > b）`，`a > b`の場合は`!（a < b`を意味します．
    伝達性: `a < b`と`b < c`は`a < c`を意味します。
    これらの要件は、トレイト自体が対称的かつ通過的に実装されなければならないことを意味していることに注意してください。`T：PartialOrd <U>`および`U：PartialOrd <V>`の場合、`U：PartialOrd <T>`および`T：PartialOrd <V>`となります。

  - Derivable
    このトレイトは、`#[derive]`と一緒に使うことができます。構造体で導出された場合、構造体のメンバの上から下への宣言順に基づいた辞書順を生成します。`enum`で導出された場合、 `variant`は、その識別順序のトップからボトムに基づいて順序付けされます。
    `PartialOrd`` partial_cmp`メソッドの実装のみを必要とし、その他の実装はデフォルトの実装から生成されます。
    しかし、全体的な順序を持たない型に対しては、その他のメソッドを個別に実装することも可能です。例えば、浮動小数点数の場合、`NaN < 0 == false`と`NaN >= 0 == false`となります (IEEE 754-2008 セクション 5.11 参照)。
    `PartialOrd`は、型が`PartialEq`である必要があります。


  - `PartialOrd`を実装するにはどうすればよいか
    型が`Ord`の場合は、`cmp`を使用して`partial_cmp`を実装することができます。

~~~rust
use std::cmp::Ordering;

#[derive(Eq)]
struct Person {
    id: u32,
    name: String,
    height: u32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Person {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}
~~~

  型のフィールドにpartial_cmpを使用すると便利です。ここでは、浮動小数点の高さのフィールドだけがソートに使用されるフィールドである Person 型の例を示します。

~~~rust
use std::cmp::Ordering;

struct Person {
    id: u32,
    name: String,
    height: f64,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.height.partial_cmp(&other.height)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}
~~~
