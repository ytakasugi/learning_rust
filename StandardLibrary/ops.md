### std::ops

- Description

  オーバーロード可能な演算子。

  これらのトレイトを実装すると、特定の演算子をオーバーロードできるようになります。

  これらのトレイトのいくつかは`prelude`でインポートされているので、すべての Rust プログラムで利用できます。トレイトに裏打ちされた演算子だけがオーバーロードできます。例えば、加算演算子(`+`)は`Add`トレイトでオーバーロードできますが、代入演算子(`=`)は`backing`トレイトがないため、そのセマンティクスをオーバーロードする方法はありません。また、本モジュールは新しい演算子を作るための仕組みを提供していません。トレイトレスのオーバーロードやカスタム演算子が必要な場合は、Rustの構文を拡張するマクロやコンパイラプラグインを検討する必要があります。

  演算子のトレイトの実装は、通常の意味と演算子の優先順位を考慮して、それぞれの文脈の中で意外性のないものでなければなりません。例えば、`Mul`を実装する場合、その演算子は乗算に似ていなければなりません（そして、連想性のような期待される特性を共有していなければなりません）。

  また，`&&`と`||`の演算子は短絡的であり，第2オペランドが結果に寄与する場合にのみ評価することに注意してください。この動作はトレイトでは強制されないので、`&&`と`||`はオーバーロード可能な演算子としてはサポートされていません。

  演算子の多くは、オペランドを値で受け取ります。組み込み型を含むジェネリックではない文脈では、通常これは問題になりません。しかし、ジェネリックなコードでこれらの演算子を使用する場合、演算子に値を消費させるのではなく、値を再利用しなければならない場合には注意が必要です。一つの方法は、`clone`を時折使用することです。もう一つの方法は、関係する型が参照のために追加の演算子の実装を提供することに依存することです。例えば、加算をサポートするとされるユーザー定義型Tの場合、`T`と`&T`の両方がトレイトの`Add<T>`と`Add<&T>`を実装することで、不必要なクローンを作らずにジェネリックなコードを書けるようにするのが良いでしょう。

- Example

  この例では、`Add`と`Sub`を実装した`Point`構造体を作成し、2つの`Point`の加算と減算を実演しています。

```rust
  use std::ops::{Add, Sub};
  
  #[derive(Debug, Copy, Clone, PartialEq)]
  struct Point {
      x: i32,
      y: i32,
  }
  
  impl Add for Point {
      type Output = Self;
  
      fn add(self, other: Self) -> Self {
          Self {x: self.x + other.x, y: self.y + other.y}
      }
  }
  
  impl Sub for Point {
      type Output = Self;
  
      fn sub(self, other: Self) -> Self {
          Self {x: self.x - other.x, y: self.y - other.y}
      }
  }
  
  assert_eq!(Point {x: 3, y: 3}, Point {x: 1, y: 0} + Point {x: 2, y: 3});
  assert_eq!(Point {x: -1, y: -3}, Point {x: 1, y: 0} - Point {x: 2, y: 3});
```

  実装例については、各トレイトのドキュメントを参照してください。

  `Fn`、`FnMut`、`FnOnce`の各トレイトは、関数のように呼び出すことができる型で実装されています。なお、`Fn`は`&self`、`FnMut`は`&mut self`、`FnOnce`は`self`を取ります。これらは、インスタンス上で呼び出すことのできる3種類のメソッドに対応しています：`call-by-reference`、`call-by-mutable-reference`、`call-by-value`です。これらのトレイトの最も一般的な使い方は、関数やクロージャを引数にとる高レベルの関数の境界として機能することです。

  `Fn`を引数にとる。

```rust
  fn call_with_one<F>(func: F) -> usize
      where F: Fn(usize) -> usize
  {
      func(1)
  }
  
  let double = |x| x * 2;
  assert_eq!(call_with_one(double), 2);
```

  `FnMut`を引数にとる。

```rust
  fn do_twice<F>(mut func: F)
      where F: FnMut()
  {
      func();
      func();
  }
  
  let mut x: usize = 1;
  {
      let add_two_to_x = || x += 2;
      do_twice(add_two_to_x);
  }
  
  assert_eq!(x, 5);
```

  `FnOnce`を引数にとる。

```rust
  fn consume_with_relish<F>(func: F)
      where F: FnOnce() -> String
  {
      // `func` consumes its captured variables, so it cannot be run more
      // than once
      println!("Consumed: {}", func());
  
      println!("Delicious!");
  
      // Attempting to invoke `func()` again will throw a `use of moved
      // value` error for `func`
  }
  
  let x = String::from("x");
  let consume_and_return_x = move || x;
  consume_with_relish(consume_and_return_x);
  
  // `consume_and_return_x` can no longer be invoked at this point
```



---

#### std::ops::Deref

- Example

  `*v`のような不変的な再参照操作に使用されます。

  `Deref`は、不変的なコンテキストでの(単項)*演算子による明示的な参照外しに使用されるだけでなく、多くの状況でコンパイラによって暗黙的に使用されます。このメカニズムは「[`Deref` coercion](https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion)」と呼ばれています。変更可能なコンテキストでは、[`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)が使用されます。

  スマートポインタに`Deref`を実装すると、その背後にあるデータへのアクセスが便利になるためです。一方で、`Deref`と[`DerefMut`](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)に関するルールは、特にスマートポインタに対応するように設計されています。このため、混乱を避けるために、`Deref`はスマートポインタに対してのみ実装されるべきです。

  同様の理由から、この特性は決して失敗してはいけません。`Deref`が暗黙のうちに起動されている場合、脱参照の際の失敗は非常に混乱を招きます。

- More on `Deref` coercion

  `T`が`Deref<Target = U>`を実装し、`x`が`T`型の値である場合。

  - 不変のコンテクストでは、`*x`（Tは参照でも生のポインタでもない）は、`*Deref::deref(&x)`と同等です。
  - `&T`型の値は`&U`型の値に強制的に変換されます。
  - Tは、型Uのすべての（不変の）メソッドを暗黙のうちに実装しています。

  詳細については、「The Rust Programming Language」の[該当の章](https://doc.rust-lang.org/book/ch15-02-deref.html)

  や、[the dereference operator](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-dereference-operator)、[method resolution](https://doc.rust-lang.org/reference/expressions/method-call-expr.html)、[type coercions](https://doc.rust-lang.org/reference/type-coercions.html)に関するリファレンスセクションを参照してください。

- Example

  構造体をデリファレンスすることでアクセス可能な1つのフィールドを持つ構造体です。

```rust
  use std::ops::Deref;
  
  struct DerefExample<T> {
      value: T
  }
  
  impl<T> Deref for DerefExample<T> {
      type Target = T;
  
      fn deref(&self) -> &Self::Target {
          &self.value
      }
  }
  
  let x = DerefExample { value: 'a' };
  assert_eq!('a', *x);
```



---

#### std::ops::Index

- Description

  不変的なコンテキストでのインデックス操作（`container[index]`）に使用されます。

  `container[index]`は、実際には`*container.index(index)`のシンタクティックシュガーですが、イミュータブルな値として使用される場合に限ります。ミュータブルな値が要求された場合は、代わりに`IndexMut`が使用されます。これにより、`value`の型が`Copy`を実装している場合、`let value = v[index]`のような素晴らしいことが可能になります。

- Example

  次の例では、読み取り専用の`NucleotideCount`コンテナに`Index`を実装し、`Index`構文で個々のカウントを取得できるようにしています。

```rust
  use std::ops::Index;
  
  enum Nucleotide {
      A,
      C,
      G,
      T,
  }
  
  struct NucleotideCount {
      a: usize,
      c: usize,
      g: usize,
      t: usize,
  }
  
  impl Index<Nucleotide> for NucleotideCount {
      type Output = usize;
  
      fn index(&self, nucleotide: Nucleotide) -> &Self::Output {
          match nucleotide {
              Nucleotide::A => &self.a,
              Nucleotide::C => &self.c,
              Nucleotide::G => &self.g,
              Nucleotide::T => &self.t,
          }
      }
  }
  
  let nucleotide_count = NucleotideCount {a: 14, c: 9, g: 10, t: 12};
  assert_eq!(nucleotide_count[Nucleotide::A], 14);
  assert_eq!(nucleotide_count[Nucleotide::C], 9);
  assert_eq!(nucleotide_count[Nucleotide::G], 10);
  assert_eq!(nucleotide_count[Nucleotide::T], 12);
```



---

#### std::ops::IndexMut

- Description

  mutableコンテキストでのインデックス操作(`container[index]`)に使用されます。

  `container[index]`は、実際には`*container.index_mut(index)`のシンタクティックシュガーですが、mutableな値として使われる場合のみです。不変的な値が要求された場合は、代わりに`Index`トレイトが使用されます。これにより、`v[index] = value`のような素晴らしいことが可能になります。

- Example

  `Balance`構造体の非常にシンプルな実装で、2つの面を持ち、それぞれが可変的および不変的にインデックスされます。

```rust
  use std::ops::{Index, IndexMut};
  
  #[derive(Debug)]
  enum Side {
      Left,
      Right,
  }
  
  #[derive(Debug, PartialEq)]
  enum Weight {
      Kilogram(f32),
      Pound(f32),
  }
  
  struct Balance {
      pub left: Weight,
      pub right: Weight,
  }
  
  impl Index<Side> for Balance {
      type Output = Weight;
  
      fn index(&self, index: Side) -> &Self::Output {
          println!("Accessing {:?}-side of balance immutably", index);
          match index {
              Side::Left => &self.left,
              Side::Right => &self.right,
          }
      }
  }
  
  impl IndexMut<Side> for Balance {
      fn index_mut(&mut self, index: Side) -> &mut Self::Output {
          println!("Accessing {:?}-side of balance mutably", index);
          match index {
              Side::Left => &mut self.left,
              Side::Right => &mut self.right,
          }
      }
  }
  
  let mut balance = Balance {
      right: Weight::Kilogram(2.5),
      left: Weight::Pound(1.5),
  };
  
  // In this case, `balance[Side::Right]` is sugar for
  // `*balance.index(Side::Right)`, since we are only *reading*
  // `balance[Side::Right]`, not writing it.
  assert_eq!(balance[Side::Right], Weight::Kilogram(2.5));
  
  // However, in this case `balance[Side::Left]` is sugar for
  // `*balance.index_mut(Side::Left)`, since we are writing
  // `balance[Side::Left]`.
  balance[Side::Left] = Weight::Kilogram(3.0);
```



---

#### std::ops::Add

- Description

  加算演算子`+`です。

  デフォルトでは`Rhs(Right Hand Side)`は`Self`であることに注意してください。例えば、`std::time::SystemTime`は`Add<Duration>`を実装しており、`SystemTime = SystemTime + Duration`という形式の操作を許可しています。

- Example

  - Addable points

    ジェネリックを使用して`Add trait`を実装した同じ`Point`構造体の例を示します。

~~~rust
    use std::ops::Add;
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add for Point {
        type Output = Self;
    
        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
~~~

    

  - Implementing Add with generics

    演算子`＋`を適用した結果の型。

~~~rust
    use std::ops::Add;
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    // Notice that the implementation uses the associated type `Output`.
    impl<T: Add<Output = T>> Add for Point<T> {
        type Output = Self;
    
        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
               Point { x: 3, y: 3 });
~~~
