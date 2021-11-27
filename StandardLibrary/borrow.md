### std::borrow

---

#### std::borrow::Borrow

- Description

  データを借用するためのtraitです。

  Rustでは、ユースケースに応じて異なる型の表現を提供するのが一般的です。例えば、`Box<T>`や`Rc<T>`のようなポインタ型によって、値の保存場所や管理方法を特定の用途に合わせて選択することができます。このように、どのような型でも使用できる汎用的なラッパーに加えて、いくつかの型では、潜在的に高価な機能を提供するオプションのファセットを提供しています。このような型の例として、基本的な`str`に文字列を拡張する機能を追加した`String`があります。これは、シンプルで不変的な文字列には不要な追加情報を保持する必要があります。

  これらの型は、データの型への参照を通じて、基礎となるデータへのアクセスを提供します。これらは、その型として「借りられる」と言われます。例えば、`Box<T>`は`T`として、`String`は`str`として借用することができます。

  型は，`Borrow<T>`を実装して，そのtraitの`borrowメ`ソッドに`T`への参照を与えることで，ある型Tとして借用できることを表現します。型は，複数の異なる型として借用するのは自由です。型として変異的に借用したい場合（基礎となるデータを変更できるようにしたい場合）は，`BorrowMut<T>`を追加で実装することができます。

  さらに、追加のトレイトのための実装を提供する際には、基礎となる型の表現として動作する結果として、基礎となる型のものと同じ動作をするべきかどうかを考慮する必要があります。一般的なコードでは、これらの追加的なトレイトの実装の同一の動作に依存する場合、`Borrow<T>`を使用します。これらのトレイトは、おそらく追加のトレイト境界として現れます。

  特に、`Eq`、`Ord`、`Hash`は、借りた値と所有している値が同じでなければなりません。`x.borrow() == y.borrow()`は、`x == y`と同じ結果になるはずです。

  一般的なコードが、関連する型Tへの参照を提供できるすべての型に対して単に動作する必要がある場合、より多くの型が安全に実装できるため、`AsRef<T>`を使用する方が良い場合が多いです。

- Example

  データコレクションとしての`HashMap<K, V>`は、キーと値の両方を所有します。キーの実際のデータが何らかの管理型に包まれている場合でも、キーのデータへの参照を使って値を検索することができるはずです。例えば、キーが文字列の場合、ハッシュマップには`String`として格納されていると思われますが、`&str`を使って検索することは可能なはずです。したがって、`insert`は`String`を操作する必要があり、`get`は`&str`を使用できる必要があります。

  少し簡略化すると、`HashMap<K, V>`の関連部分は次のようになります。

```rust
  use std::borrow::Borrow;
  use std::hash::Hash;
  
  pub struct HashMap<K, V> {
      // fields omitted
  }
  
  impl<K, V> HashMap<K, V> {
      pub fn insert(&self, key: K, value: V) -> Option<V>
      where K: Hash + Eq
      {
          // ...
      }
  
      pub fn get<Q>(&self, k: &Q) -> Option<&V>
      where
          K: Borrow<Q>,
          Q: Hash + Eq + ?Sized
      {
          // ...
      }
  }
```

  これらのキーはハッシュマップと共に保存されるため、このタイプはキーのデータを所有していなければならない。キーと値のペアを挿入するとき、マップにはこのような`K`が与えられ、正しいハッシュバケットを見つけ、その`K`に基づいてキーがすでに存在するかどうかをチェックする必要があります。

  しかし、マップで値を検索する際に、検索するキーとして`K`への参照を提供しなければならないと、常にそのような所有する値を作成する必要があります。文字列キーの場合、`str`しかない場合には、検索のためだけに`String`の値を作成する必要があるということになります。

  その代わりに、getメソッドは、上記のメソッドシグネチャで`Q`と呼ばれる、基礎となるキーデータの型に対してジェネリックになります。`K: Borrow<Q>`とすることで、`K`が`Q`として借用することを表明しています。さらに`Q: Hash + Eq`を要求することで、`K`と`Q`が同一の結果をもたらす`Hash`と`Eq`のトレイトの実装を持っていることを要求しています。

  `get`の実装は、特に`Hash`の同一の実装に依存しており、`K`の値から計算されたハッシュ値に基づいてキーを挿入したにもかかわらず、`Q`の値で`Hash::hash`を呼び出してキーのハッシュバケットを決定しています。

  その結果、`Q`値をラップした`K`が`Q`とは異なるハッシュを生成した場合、ハッシュマップは壊れてしまう。例えば、文字列をラップするが、ASCII文字を大文字小文字を無視して比較する型があるとする。

```rust
  pub struct CaseInsensitiveString(String);
  
  impl PartialEq for CaseInsensitiveString {
      fn eq(&self, other: &Self) -> bool {
          self.0.eq_ignore_ascii_case(&other.0)
      }
  }
  
  impl Eq for CaseInsensitiveString { }
```

  2つの等しい値が同じハッシュ値を生成する必要があるため、`Hash`の実装ではASCIIの大文字小文字を無視する必要があります。

```rust
  impl Hash for CaseInsensitiveString {
      fn hash<H: Hasher>(&self, state: &mut H) {
          for c in self.0.as_bytes() {
              c.to_ascii_lowercase().hash(state)
          }
      }
  }
```

  `CaseInsensitiveString`は`Borrow<str>`を実装できますか？確かに、含まれる所有文字列を介して、文字列スライスへの参照を提供することができます。しかし、`Hash`の実装が異なるため、`str`とは挙動が異なり、実際には`Borrow<str>`を実装してはいけません。もし他の人に基礎となる`str`へのアクセスを許可したいのであれば、余分な要件を持たない`AsRef<str>`を介して行うことができます。

- Required methods

  - ### std::borrow::Borrow::borrow

    - Description

      Immutably borrows from an owned value.

    - Example

    ```rust
      use std::borrow::Borrow;
      
      fn check<T: Borrow<str>>(s: T) {
          assert_eq!("Hello", s.borrow());
      }
      
      let s = "Hello".to_string();
      
      check(s);
      
      let s = "Hello";
      
      check(s);
    ```



---

#### std::borrow::ToOwned::to_owned

- Description

  借用したデータから所有するデータを作成します。通常はクローンを作成します。

- Example

```rust
  let s: &str = "a";
  let ss: String = s.to_owned();
  
  let v: &[i32] = &[1, 2];
  let vv: Vec<i32> = v.to_owned();
```
