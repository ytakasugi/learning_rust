### std::cell

- Description

  共有可能な可変型コンテナ。

  Rustのメモリ安全性はこのルールに基づいています。オブジェクトTが与えられた場合、以下のいずれかの可能性しかありません。

  - オブジェクトに対する複数の不変的な参照(`&T`)を持つ(エイリアシングとも呼ばれる)。
  - オブジェクトに対する1つの可変型参照(`&mut T`)を持つこと(可変型とも呼ばれます)。

  これはRustのコンパイラによって強制されています。しかし、このルールが十分に柔軟ではない状況もあります。あるオブジェクトへの複数の参照を持ち、なおかつそれを可変させることが必要な場合があります。

  エイリアシングがあっても、制御された方法でミュータビリティを可能にするために、共有可能なミュータブルコンテナが存在します。`Cell<T>`と`RefCell<T>`の両方は、シングルスレッドの方法でこれを行うことができます。しかし、`Cell<T>`も`RefCell<T>`もスレッドセーフではありません (`Sync`を実装していません)。複数のスレッド間でエイリアシングやミューテーションを行う必要がある場合は、`Mutex<T>`、`RwLock<T>`、またはアトミックタイプを使用することが可能です。

  `Cell<T>`と`RefCell<T>`型の値は、共有参照 (すなわち、共通の`&T`型) を通して変異させることができますが、ほとんどの Rustの型は、一意の (`&mut T`) 参照を通してのみ変異させることができます。`Cell<T>`と`RefCell<T>`は「内部の可変性」を提供していると言い、典型的なRustの型が「継承された可変性」を示すのとは対照的です。

  `Cell`型には2つの種類があります。`Cell<T>`と`RefCell<T>`です。`Cell<T>`は、`Cell<T>`の中に値を出し入れすることで、内部ミュータビリティを実現します。値の代わりに参照を使用するには、`RefCell<T>`型を使用し、ミューティングの前に書き込みロックを取得する必要があります。`Cell<T>`には、現在の内部の値を取得したり変更したりするメソッドが用意されています。

  - `Copy`を実装している型では、`get`メソッドが現在の内部値を取得します。
  - `Default`を実装している型の場合、`take`メソッドは現在の内部値を`Default::default()`で置き換え、置き換えられた値を返します。
  - すべての型では、`replace`メソッドは現在の内部の値を置き換えて、置き換えられた値を返し、`into_inner`メソッドは`Cell<T>`を消費して内部の値を返します。さらに、`set`メソッドは内部の値を置き換え、置き換えられた値をドロップします。

  `RefCell<T>`はRustの寿命を利用して「動的な借用」を実装しており、内部の値への一時的、排他的、可変型のアクセスを主張することができます。`RefCell<T>`のボローは「実行時」に追跡されますが、Rustのネイティブな参照型は完全にコンパイル時に静的に追跡されるのとは異なります。`RefCell<T>`の借用は動的であるため、すでに変異可能に借用されている値を借用しようとすることが可能です。これが起こるとスレッドパニックになります。

- When to choose interior mutability

  より一般的な継承された可変型では、値を変更するためには一意にアクセスしなければなりませんが、これはRustがポインタのエイリアシングを強力に推論し、クラッシュバグを静的に防止するための重要な言語要素のひとつです。このような理由から、継承された変異性が好まれ、内部の変異性は最終手段のようなものです。しかし、セルタイプは他の方法では許されない場所での変異を可能にするので、内部の可変性が適切であるか、あるいは使用しなければならない場合があります。

  - 不変的なものの「内部」に突然変異性を導入する
  - 論理的に不変のメソッドの実装の詳細。
  - `Clone`の実装を変更する。

- Introducing mutability ‘inside’ of something immutable

  `Rc<T>`や`Arc<T>`をはじめとする多くの共有スマートポインタ型は、複数の関係者間でクローン化して共有できるコンテナを提供しています。含まれる値はマルチプライス・エイリアスがかかっている可能性があるため、`&mut`ではなく`&`でしか借用できません。セルがなければ、これらのスマートポインタ内のデータを変異させることはまったくできません。

  そこで、共有されたポインタ型の中に`RefCell<T>`を入れて、ミュータビリティを再導入することがよく行われます。

```rust
  use std::cell::{RefCell, RefMut};
  use std::collections::HashMap;
  use std::rc::Rc;
  
  fn main() {
      let shared_map: Rc<RefCell<_>> = Rc::new(RefCell::new(HashMap::new()));
      // Create a new block to limit the scope of the dynamic borrow
      {
          let mut map: RefMut<_> = shared_map.borrow_mut();
          map.insert("africa", 92388);
          map.insert("kyoto", 11837);
          map.insert("piccadilly", 11826);
          map.insert("marbles", 38);
      }
  
      // Note that if we had not let the previous borrow of the cache fall out
      // of scope then the subsequent borrow would cause a dynamic thread panic.
      // This is the major hazard of using `RefCell`.
      let total: i32 = shared_map.borrow().values().sum();
      println!("{}", total);
  }
```

  この例では`Arc<T>`ではなく`Rc<T>`を使用していることに注意してください。`RefCell<T>`はシングルスレッドのシナリオ用です。マルチスレッドの状況で共有のミュータビリティが必要な場合は、`RwLock<T>`または`Mutex<T>`の使用を検討してください。

- Implementation details of logically-immutable methods

  場合によっては、「ボンネットの中」で突然変異が起こっていることを API で公開しないことが望まれることがあります。これは、論理的には操作は不変ですが、例えばキャッシングのために実装がミューテーションを行わざるを得ない場合や、もともと`&self`を取るように定義されていたtraitメソッドを実装するためにミューテーションを採用しなければならない場合などが考えられます。

```rust
  use std::cell::RefCell;
  
  struct Graph {
      edges: Vec<(i32, i32)>,
      span_tree_cache: RefCell<Option<Vec<(i32, i32)>>>
  }
  
  impl Graph {
      fn minimum_spanning_tree(&self) -> Vec<(i32, i32)> {
          self.span_tree_cache.borrow_mut()
              .get_or_insert_with(|| self.calc_span_tree())
              .clone()
      }
  
      fn calc_span_tree(&self) -> Vec<(i32, i32)> {
          // Expensive computation goes here
          vec![]
      }
  }
```

- Mutating implementations of `Clone`

  これは、不変であるかのように見える操作のために可変性を隠すという、先ほどの特殊な（しかし一般的な）ケースに過ぎません。`clone`メソッドは、ソースの値を変更しないことが期待されており、`&mut self`ではなく、`&self`を取るように宣言されています。そのため、`clone`メソッドで発生する変異は、セル型を使用しなければなりません。例えば、`Rc<T>`は`Cell<T>`の中で参照カウントを維持します。

```rust
  use std::cell::Cell;
  use std::ptr::NonNull;
  use std::process::abort;
  use std::marker::PhantomData;
  
  struct Rc<T: ?Sized> {
      ptr: NonNull<RcBox<T>>,
      phantom: PhantomData<RcBox<T>>,
  }
  
  struct RcBox<T: ?Sized> {
      strong: Cell<usize>,
      refcount: Cell<usize>,
      value: T,
  }
  
  impl<T: ?Sized> Clone for Rc<T> {
      fn clone(&self) -> Rc<T> {
          self.inc_strong();
          Rc {
              ptr: self.ptr,
              phantom: PhantomData,
          }
      }
  }
  
  trait RcBoxPtr<T: ?Sized> {
  
      fn inner(&self) -> &RcBox<T>;
  
      fn strong(&self) -> usize {
          self.inner().strong.get()
      }
  
      fn inc_strong(&self) {
          self.inner()
              .strong
              .set(self.strong()
                       .checked_add(1)
                       .unwrap_or_else(|| abort() ));
      }
  }
  
  impl<T: ?Sized> RcBoxPtr<T> for Rc<T> {
     fn inner(&self) -> &RcBox<T> {
         unsafe {
             self.ptr.as_ref()
         }
     }
  }
```



---

#### std::cell::Cell

- Description

  ミュータブルなメモリロケーション。

- Example

  この例では、`Cell<T>`が不変の構造体の内部で変異を可能にしていることがわかります。言い換えれば、"interior mutability "を実現しています。

```rust
  use std::cell::Cell;
  
  struct SomeStruct {
      regular_field: u8,
      special_field: Cell<u8>,
  }
  
  let my_struct = SomeStruct {
      regular_field: 0,
      special_field: Cell::new(1),
  };
  
  let new_value = 100;
  
  // ERROR: `my_struct` is immutable
  // my_struct.regular_field = new_value;
  
  // WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
  // which can always be mutated
  my_struct.special_field.set(new_value);
  assert_eq!(my_struct.special_field.get(), new_value);
```



---

#### std::cell::Cell::set

- Description

  含まれる値を設定します。

- Example

```rust
  use std::cell::Cell;
  
  let c = Cell::new(5);
  
  c.set(10);
```



---

#### std::cell::Cell::get

- Description

  含まれる値のコピーを返します。

- Example

```rust
  use std::cell::Cell;
  
  let c = Cell::new(5);
  
  let five = c.get();
```



---

#### std::cell::Refcell

- Description

  動的にチェックされた借用ルールを持つ、変更可能なメモリロケーション

  詳細は、モジュールレベルの[ドキュメント](https://doc.rust-lang.org/stable/std/cell/index.html)をご覧ください。



---

#### std::cell::RefCell::new

- Description

  値を含む新しい`RefCell`を作成します。

- Example

```rust
  use std::cell::RefCell;
  
  let c = RefCell::new(5);
```



---

#### std::cell::RefCell::borrow_mut

- Description

  ラップされた値を相互に借用します。

  この借り入れは、返された`RefMut`またはそれから派生したすべての`RefMut`がスコープを出るまで続きます。この借用が有効な間は、その値を借用することはできません。

- Panics

  値が現在借用されている場合、パニックになります。パニックを起こさないようにするには`try_borrow_mut`を使います。

- Example

```rust
  use std::cell::RefCell;
  
  let c = RefCell::new("hello".to_owned());
  
  *c.borrow_mut() = "bonjour".to_owned();
  
  assert_eq!(&*c.borrow(), "bonjour");
```

  An example of panic:

```rust
  use std::cell::RefCell;
  
  let c = RefCell::new(5);
  let m = c.borrow();
  
  let b = c.borrow_mut(); // this causes a panic
```



---

#### std::cell::RefCell::borrow

- Example

  ラップされた値を不変に借用します。

  借用は、返された Ref がスコープを出るまで続きます。複数の`immutable borrows`を同時に取り出すことができます。

- Panics

  値が現在`mutably`に借用されている場合、パニックになります。パニックを起こさないようにするには`try_borrow`を使ってください。

- Example

```rust
  use std::cell::RefCell;
  
  let c = RefCell::new(5);
  
  let borrowed_five = c.borrow();
  let borrowed_five2 = c.borrow();
```

  An example panic:

```rust
  use std::cell::RefCell;
  
  let c = RefCell::new(5);
  
  let m = c.borrow_mut();
  let b = c.borrow(); // this causes a panic
```



---

#### std::cell::RefCell::try_borrow_mut

- Description

  ラップされた値を相互に借用し、その値が現在借用されている場合はエラーを返します。

  借用は、返された`RefMut`またはその`RefMut`から派生したすべての`RefMut`がスコープを出るまで続きます。この借用が有効な間は、値を借用することはできません。

  これは、`borrow_mut`のパニックにならないバリアントです。

- Example

```rust
  use std::cell::RefCell;
  
  let c = RefCell::new(5);
  
  {
      let m = c.borrow();
      assert!(c.try_borrow_mut().is_err());
  }
  
  assert!(c.try_borrow_mut().is_ok());
```
