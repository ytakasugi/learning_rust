### std::rc

 - Description

   シングルスレッドの参照カウントポインタ。`Rc`は`Reference Counted`の略。

   `Rc<T>`型は、ヒープに割り当てられたT型の値の共有所有権を提供する。`Rc`上で`clone`を実行すると、ヒープ内の同じ値への新しいポインタが生成される。与えられたアロケーションへの最後の`Rc`ポインタが破棄されると、そのアロケーションに格納されている値(しばしば "内部値 "と呼ばれる)も破棄される。
   Rustの共有参照はデフォルトで突然変更されることを禁止しており、`Rc`も例外ではない。もし値の変更が必要な場合は、`Rc`の中に`Cell`や`RefCell`を包含してください。
   `Rc`は非アトミックな参照カウントを使用する。これはオーバーヘッドが非常に低いことを意味しますが、`Rc`はスレッド間で送信することができないため、`Rc`は`Send`を実装していない。その結果、Rustコンパイラはコンパイル時にスレッド間で`Rcs`を送信していないかどうかをチェックする。マルチスレッドでアトミックな参照カウントが必要な場合は、`sync::Arc`を使用すること。
   `downgrade`メソッドを使用して、所有していない`Weak`ポインタを作成することができる。`Weak`ポインタを`Rc`にアップグレードすることができますが、アロケーションに格納されている値が既にドロップされている場合は`None`を返します。言い換えれば、`Weak`ポインタはアロケーション内の値を保持しない。
   `Rc`ポインタ間のサイクルは決して解放されない。このため、`Weak`はサイクルを壊すために使用される。例えば、ツリーは親ノードから子ノードへの強い`Rc`ポインターを持ち、子ノードから親ノードへの弱いポインターを持つことができる。
   `Rc<T>`は自動的にTへの派生を行います（`Deref trait`で）ので、`Rc<T>`型の値で`T`のメソッドを呼び出すことができます。`T`のメソッドとの名前の衝突を避けるために、`Rc<T>`のメソッドは関連する関数であり、[完全修飾構文]で呼ばれる。

 ~~~rust
   use std::rc::Rc;
   
   let my_rc = Rc::new(());
   Rc::downgrade(&my_rc);
 ~~~

   `Clone`のようなトレイトの`Rc<T>`の実装も完全修飾構文を使って呼ばれることがあります。完全修飾構文を好む人もいれば、メソッド呼び出し構文を好む人もいます。

 ~~~rust
   use std::rc::Rc;
   
   let rc = Rc::new(());
   // Method-call syntax
   let rc2 = rc.clone();
   // Fully qualified syntax
   let rc3 = Rc::clone(&rc);
 ~~~

   `Weak<T>`は、内部の値が既に落とされている可能性があるため、Tへの自動参照は行わない。

   - Cloning references
     既存の参照カウントポインタと同じアロケーションへの新しい参照の作成は、`Rc<T>`と`Weak<T>`のために実装されたClone traitを使用して行われる。

 ~~~rust
   use std::rc::Rc;
   
   let foo = Rc::new(vec![1.0, 2.0, 3.0]);
   // The two syntaxes below are equivalent.
   let a = foo.clone();
   let b = Rc::clone(&foo);
   // a and b both point to the same memory location as foo.
 ~~~

   `Rc::clone(&from)`構文は、コードの意味をより明確に伝えることができるので、最も慣用的である。上の例では、この構文を使うと、このコードが`foo`の内容を丸ごとコピーするのではなく、新しい参照を作成していることがわかりやすくなる。

   - Example
     あるガジェットを所有者が所有している場合を考えてみる。ガジェットの所有者を特定できるようにしたいが、所有者を特定することはできない。しかし、複数のガジェットが同じオーナーに属している可能性があるため、ユニークなオーナーシップではこれを行うことができない。`Rc`では複数のガジェット間でオーナーを共有し、どのガジェットがポイントしている間もオーナーが割り当てられたままにしておくことができる。

 ~~~rust
   use std::rc::Rc;
   
   struct Owner {
       name: String,
       // ...other fields
   }
   
   struct Gadget {
       id: i32,
       owner: Rc<Owner>,
       // ...other fields
   }
   
   fn main() {
       // Create a reference-counted `Owner`.
       let gadget_owner: Rc<Owner> = Rc::new(
           Owner {
               name: "Gadget Man".to_string(),
           }
       );
   
       // Create `Gadget`s belonging to `gadget_owner`. Cloning the `Rc<Owner>`
       // gives us a new pointer to the same `Owner` allocation, incrementing
       // the reference count in the process.
       let gadget1 = Gadget {
           id: 1,
           owner: Rc::clone(&gadget_owner),
       };
       let gadget2 = Gadget {
           id: 2,
           owner: Rc::clone(&gadget_owner),
       };
   
       // Dispose of our local variable `gadget_owner`.
       drop(gadget_owner);
   
       // Despite dropping `gadget_owner`, we're still able to print out the name
       // of the `Owner` of the `Gadget`s. This is because we've only dropped a
       // single `Rc<Owner>`, not the `Owner` it points to. As long as there are
       // other `Rc<Owner>` pointing at the same `Owner` allocation, it will remain
       // live. The field projection `gadget1.owner.name` works because
       // `Rc<Owner>` automatically dereferences to `Owner`.
       println!("Gadget {} owned by {}", gadget1.id, gadget1.owner.name);
       println!("Gadget {} owned by {}", gadget2.id, gadget2.owner.name);
   
       // At the end of the function, `gadget1` and `gadget2` are destroyed, and
       // with them the last counted references to our `Owner`. Gadget Man now
       // gets destroyed as well.
   }
 ~~~

   しかし、要求が変化してオーナーからガジェットへの移動が必要になった場合、問題が発生することになる。オーナーからガジェットへの `Rc`ポインタはサイクルを導入する。これは、それらの参照カウントが`0`になることはなく、アロケーションが破棄されることもないことを意味する。これを回避するために、`Weak`ポインタを使うことができます。

   Rustは実際には、そもそもこのループを生成することをやや難しくしている。2つの値がお互いを指すようになるためには、そのうちの1つは変更可能である必要があります。これは、`Rc`がラップした値への共有参照のみを与えることでメモリの安全性を確保しており、直接の突然変異を許さないからである。これは内部可変性を提供する`RefCell`で、共有参照を介して変異性を実現する方法。`RefCell`は実行時にRustの借用ルールを強制する。



---

#### std::rc::Rc

- Description

  シングルスレッドの参照カウント式ポインター。`Rc`は「Reference Counted」の略です。

  詳細は、モジュールレベルのドキュメントを参照してください。

  `Rc`の固有のメソッドはすべて関連する関数であり、`value.get_mut()`ではなく、例えば`Rc::get_mut(&mut value)`のように呼ばなければならないことを意味します。これにより、内部型Tのメソッドとの衝突を避けることができます。



---

#### std::rc::Rc::new

- Description

  新しい`Rc<T>`を構築します。

- Example

```rust
  use std::rc::Rc;
  
  let five = Rc::new(5);
```



---

#### std::rc::Rc::strong_count

- Description

  このアロケーションに対する強い(Rc)ポインタの数を取得します。

- Example

```rust
  use std::rc::Rc;
  
  let five = Rc::new(5);
  let _also_five = Rc::clone(&five);
  
  assert_eq!(2, Rc::strong_count(&five));
```



---

#### std::rc::Rc::clone

- Description

  `Rc`ポインタのクローンを作成します。

  これにより、同じアロケーションへの別のポインタが作成され、強い参照カウントが増加します。

- Example

```rust
  use std::rc::Rc;
  
  let five = Rc::new(5);
  
  let _ = Rc::clone(&five);
```




---

#### std::rc::Rc::get_mut

- Description

  同じアロケーションへの他の`Rc`または`Weak`ポインタがない場合，与えられた`Rc`への可変な参照を返します。

  そうでなければ`None`を返します。なぜなら、共有された値を変異させることは安全ではないからです。

  他のポインタがある場合に内部の値をクローン化する`make_mut`も参照してください。

- Example

```rust
  use std::rc::Rc;
  
  let mut x = Rc::new(3);
  *Rc::get_mut(&mut x).unwrap() = 4;
  assert_eq!(*x, 4);
  
  let _y = Rc::clone(&x);
  assert!(Rc::get_mut(&mut x).is_none());
```



---

#### std::rc::Rc::downgrade

- Description

  このアロケーションへの新しい`Weak`ポインタを作成します。

- Example

```rust
  use std::sync::Arc;
  
  let five = Arc::new(5);
  
  let weak_five = Arc::downgrade(&five);
```



---

#### std::rc::Weak::upgrade

- Description

  `Weak`ポインタを`Rc`にアップグレードしようと試み，成功すれば内部値のドロップを遅らせる。

  内側の値がドロップされた後であれば`None`を返します。

- Example

```rust
  use std::rc::Rc;
  
  let five = Rc::new(5);
  
  let weak_five = Rc::downgrade(&five);
  
  let strong_five: Option<Rc<_>> = weak_five.upgrade();
  assert!(strong_five.is_some());
  
  // Destroy all strong pointers.
  drop(strong_five);
  drop(five);
  
  assert!(weak_five.upgrade().is_none());
```
