# 標準ライブラリ

本テキストは、Rustを学習していく中で使用する標準ライブラリについて、記載していく。

詳細は、[標準ライブラリの公式ドキュメント](https://doc.rust-lang.org/stable/std/)を参照のこと。

---

### 参照の規則

- 任意のタイミングで、一つの可変参照か任意の数の不変参照を作れる
- 参照は常に有効でなければならない

---

### ライフタイムの省略規則

- 関数の戻り値の型が参照型のとき
- 引数の中で参照型が1つだけなら、その型から借用する
- 第1引数が`&self`または`&mut self`のメソッドなら、(他に参照型の引数があっても)`self`から借用する
- それ以外の場合は、ライフタイムは省略できない

---

### Selfキーワード

  - Description
    トレイトまたは`impl`ブロック内の実装型、または型定義内の現在の型。

---

### selfキーワード

  - Description

    メソッドのレシーバー、または現在のモジュール。
    selfは2つの状況で使用されます。

    - カレントモジュールを参照すること

    - メソッドのレシーバーをマークすることです。

    パスでは、use文の中で、あるいは要素にアクセスするパスの中で、selfは現在のモジュールを    参照するために使用されます。

---

### dynキーワード

  - Description

    `dyn` は trait オブジェクトの型の接頭辞である

    `dyn`キーワードは、関連付けられた Trait のメソッドの呼び出しが動的にディスパッチされることを強調するために使用する。このように trait を使用するには、それが `object safe` である必要がある。
    一般的なパラメータや`impl Trait`とは異なり、コンパイラは渡される具体的な型を推論できない。つまり、型は消去されている。そのため、`dyn Trait`参照には2つのポインタが含まれている。
    1つのポインタはデータ(構造体のインスタンスなど)へのポインタ。もう 1 つのポインタは、メソッド呼び出し名と関数ポインタのマップ(仮想メソッドテーブルまたは`vtable`として知られている)へのポインタです。
    実行時に、`dyn Trait`上でメソッドを呼び出す必要がある場合、関数ポインタを取得するために`vtable`が参照され、その関数ポインタが呼び出される。

    - トレードオフ

      上記は間接的には、`dyn Trait`上で関数を呼び出す際の追加の実行コストである。動的ディスパッチによって呼び出されたメソッドは、一般的にコンパイラによってインライン化することができない。
      しかし、具体的な型ごとにメソッドが重複しないため、`dyn Trait`は`impl Trait / generic parameters` よりも小さなコードを生成する可能性がある。
      オブジェクトの安全性と traitオブジェクトについての詳細は[こちら](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html)を参照。

---

### [オブジェクト安全性](https://doc.rust-jp.rs/book-ja/ch17-02-trait-objects.html#%E3%83%88%E3%83%AC%E3%82%A4%E3%83%88%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%E3%81%AB%E3%81%AF%E3%82%AA%E3%83%96%E3%82%B8%E3%82%A7%E3%82%AF%E3%83%88%E5%AE%89%E5%85%A8%E6%80%A7%E3%81%8C%E5%BF%85%E8%A6%81)

トレイトオブジェクトには、*オブジェクト安全*なトレイトしか作成できません。 トレイトオブジェクトを安全にする特性全てを司る複雑な規則がありますが、実際には、2つの規則だけが関係があります。 トレイトは、トレイト内で定義されているメソッド全てに以下の特性があれば、オブジェクト安全になります。

- 戻り値の型が`Self`でない。
- ジェネリックな型引数がない。

`Self`キーワードは、トレイトやメソッドを実装しようとしている型の別名です。トレイトオブジェクトは、 一旦、トレイトオブジェクトを使用したら、コンパイラにはそのトレイトを実装している具体的な型を知りようがないので、 オブジェクト安全でなければなりません。トレイトメソッドが具体的な`Self`型を返すのに、 トレイトオブジェクトが`Self`の具体的な型を忘れてしまったら、メソッドが元の具体的な型を使用できる手段はなくなってしまいます。 同じことがトレイトを使用する時に具体的な型引数で埋められるジェネリックな型引数に対しても言えます: 具体的な型がトレイトを実装する型の一部になるのです。トレイトオブジェクトの使用を通して型が忘却されたら、 そのジェネリックな型引数を埋める型がなんなのか知る術はありません。

---

### refキーワード

- Description

  パターンマッチングの際に参照によってバインドします。

  `ref`は、パターンのバインディングにアノテーションを付けて、移動ではなく借用するようにします。マッチングに関する限りでは、これはパターンの一部ではありません。

  デフォルトでは、`match`文は可能な限りの値を消費しますが、値を移動して所有する必要がない場合には問題になることがあります。

~~~rust
  let maybe_name = Some(String::from("Alice"));
  // The variable 'maybe_name' is consumed here ...
  match maybe_name {
      Some(n) => println!("Hello, {}", n),
      _ => println!("Hello, world"),
  }
  // ... and is now unavailable.
  println!("Hello again, {}", maybe_name.unwrap_or("world".into()));
~~~

  `ref`キーワードを使用すると、値は借用されるだけで、移動されることはありません。

~~~rust
  let maybe_name = Some(String::from("Alice"));
  // Using `ref`, the value is borrowed, not moved ...
  match maybe_name {
      Some(ref n) => println!("Hello, {}", n),
      _ => println!("Hello, world"),
  }
  // ... so it's available here!
  println!("Hello again, {}", maybe_name.unwrap_or("world".into()));
~~~

- `&` vs `ref`

  - `&`パターンがオブジェクトへの参照を期待していることを示しています。したがって`&`はパターンの一部です: `&Foo`は`Foo`とは異なるオブジェクトにマッチします。

  - `ref`は、アンパックされていない値への参照を求めていることを示します。対してはマッチしません。`Foo(ref foo)`は`Foo(foo)`と同じオブジェクトにマッチします。

    

  詳細は[Reference](https://doc.rust-lang.org/stable/reference/patterns.html#identifier-patterns)も参照してください。

---

### 変数の状態と可能な操作の一覧

| 変数xの状態 | 変数xの使用/借用 | 変数xへの代入 | 変数xの可変参照 | 変数xからのムーブ |
| ----------- | ---------------- | ------------- | --------------- | ----------------- |
| 未初期化    | ×                | 〇            | ×               | ×                 |
| 不変変数    | 〇               | ×             | ×               | 〇                |
| 可変変数    | 〇               | 〇            | 〇              | 〇                |
| *(不変参照) | 〇               | ×             | ×               | ×                 |
| *(可変参照) | 〇               | 〇            | 〇              | ×                 |
| 借用中      | 〇               | ×             | ×               | ×                 |
| 可変借用中  | ×                | ×             | ×               | ×                 |
| ムーブ後    | ×                | 〇            | ×               | ×                 |

---

### String

  - Description

    UTF-8エンコードされた可変長文字列

    String型は、文字列の内容を所有する最も一般的な文字列型。これは、その借用型であるプリミティブ 型のstr型 と密接な関係を持っている。

    表.String型とstr型

    | 型       | 役割                | 実データを格納するメモリ領域                               | 文字の変更 | 文字の追加・削除 | 実データを所有しているか |
    | -------- | ------------------- | ---------------------------------------------------------- | ---------- | ---------------- | ------------------------ |
    | String   | 可変長のUTF-8文字列 | ヒープ領域                                                 | 可         | 可               | 所有する                 |
    | &str     | 固定長のUTF-8文字列 | ヒープ領域、スタック領域、静的領域のいずれか。参照先に依存 | 不可       | 不可             | 所有しない               |
    | &mut str | 固定長のUTF-8文字列 | ヒープ領域、スタック領域、静的領域のいずれか。参照先に依存 | 可         | 不可             | 所有しない               |

    ※&strは不変スライス経由のアクセス、&mut strは可変スライス経由のアクセス

---


### 配列を表現する型

  - Description

    以下に配列を表現する型として、配列、スライス、ベクタについてまとめる

    表.配列を表現する型

    | 型                                     | 役割                             | 実データを格納するメモリ領域                 | 要素数が決定されるタイミング | 要素の追加・削除 | 実データを所有するか |
    | -------------------------------------- | -------------------------------- | -------------------------------------------- | ---------------------------- | ---------------- | -------------------- |
    | ベクタ`Vec<T>`                         | サイズ可変の配列                 | ヒープ領域                                   | 実行時                       | 可               | 所有する             |
    | 配列`[T; n]`                           | サイズ固定の配列                 | スタック領域                                 | コンパイル時(型に現れる)     | 不可             | 所有する             |
    | `ボックス化されたスライスBox<[T]>`     | サイズ固定の配列                 | ヒープ領域                                   | 実行時                       | 不可             | 所有する             |
    | そのほかのスライス(`&[T]`、`&mut [T]`) | ベクタや配列へのアクセスを抽象化 | ヒープ領域、またはスタック領域。参照先に依存 | 実行時                       | 不可             | 所有しない           |

---

### クロージャが実装するトレイト

- Descriptiom

  クロージャが実装するトレイトには、Fn、FnMut、FnOnceの3つがあり、どれを実装するかは環境に補足した外部の変数(自由変数)をクロージャの本体がどのように扱うかで決まる。

  表.クロージャが実装するトレイト(〇：実装する、×：実装しない)

  | 環境を表す匿名構造体の使い方                                 | [Fn](https://doc.rust-lang.org/stable/std/ops/trait.Fn.html) | [FnMut](https://doc.rust-lang.org/stable/std/ops/trait.FnMut.html) | [FnOnce](https://doc.rust-lang.org/stable/std/ops/trait.FnOnce.html) |
  | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
  | 環境が空(何も補足していない)                                 | 〇                                                           | 〇                                                           | 〇                                                           |
  | 読むだけ。すべてのフィールドには不変の参照(&T)経由でアクセス | 〇                                                           | 〇                                                           | 〇                                                           |
  | 変更する。1つ以上のフィールドに可変の参照(&mut T)経由でアクセス、かつ所有権をムーブするフィールドがない | ×                                                            | 〇                                                           | 〇                                                           |
  | 消費する。1つ以上のフィールドからクロージャの本体へ所有権をムーブする | ×                                                            | ×                                                            | 〇                                                           |

- ### Fn

  - Description

    `Fn`トレイトはクロージャが不変の環境を持つことを示す。`Fn`トレイトを実装するクロージャは何度でも実行でき、たとえば`Sync`トレイトを実装すれば、複数のスレッドでの同時実行もできる。`FnMut`と`FnOnce`の両方が`Fn`のスーパートレイトであるため、`Fn`トレイトを実装するクロージャは`FnMut`と`FnOnce`トレイトも実装するので、それらが要求される箇所でも使える。

    - Example

~~~rust
      fn call_with_one<F>(func: F) -> usize
          where F: Fn(usize) -> usize {
          func(1)
      }
      
      let double = |x| x * 2;
      assert_eq!(call_with_one(double), 2);
~~~

- ### FnMut

  - Description

    `FnMut`トレイトはクロージャが可変の環境持つことを示す。`FnMut`トレイトを実装するクロージャは何度でも実行できますが、複数スレッドで同時実行するには、クロージャだけでなく環境のすべての型が`Sync`トレイトを実装している必要がある。`FnOnce`は`FnMut`のスーパートレイトなので、`FnMut`を実装するクロージャは`FnOnce`も実装するので、それらが要求される箇所でも使える。

    - Example

~~~rust
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
~~~

- ### FnOnce

  - Description

    `FnOnce`トレイトは、環境からクロージャの本体へ所有権がムーブすることを示す。そのため、`FnOnce`トレイトを実装したクロージャは一度しか実行できない。

    - Example

~~~rust
      fn consume_with_relish<F>(func: F)
          where F: FnOnce() -> String
      {
          // `func` consumes its captured variables, so it cannot be run more
          // than once.
          println!("Consumed: {}", func());
      
          println!("Delicious!");
      
          // Attempting to invoke `func()` again will throw a `use of moved
          // value` error for `func`.
      }
      
      let x = String::from("x");
      let consume_and_return_x = move || x;
      consume_with_relish(consume_and_return_x);
      
      // `consume_and_return_x` can no longer be invoked at this point
~~~

---

### マーカートレイト

トレイトの中には、メソッドのない。それぞれの持つ意味や役割をしるしのように付与するものがあります。そのようなトレイトを、マーカートレイトといいます。`Copy`や`Sized`などのトレイトがそれにあたります。

以下にマーカートレイトの一部を列挙します。

- `Copy`

  値の所有権を渡す代わりに、値のコピーを行うようにする。

- `Send`

  スレッド境界を越えて所有権を転送できることを示す。

- `Sized`

  メモリ上でサイズが決まっていることを示す。

- `Sync`

  スレッド間で安全に参照を共有できることを示す。

これらのトレイトはデータを含んでいないため、実行時にもメモリ内にデータが存在しませんが、コンパイラが安全性の検査や最適化する際に使用されます。



---

### [std::clone::Clone](https://doc.rust-lang.org/stable/std/clone/trait.Clone.html)

  - Description

    オブジェクトを明示的に複製することができる共通のトレイト

    Copyとの違いは、Copyは暗黙的で非常に安価であるのに対して、Cloneは常に明示的あり、高価であるときもあればそうでないときもある点である。これらの特徴を強制するために、RustではCopyは再実装できませんが、Cloneを再実装して任意のコードに対してい再実行できる。

    CloneはCopyよりも一般的なので、Copyであればなんでも自動的にCloneにすることができる。

    このトレイトは、すべてのフィールドがCloneであれば#[derive]で使用できます。Cloneの派生実装は、各フィールドでcloneを呼び出す。

    一般的な構造体の場合、#[derive]は一般的なパラメータにバインドされたCloneを追加することで条件付きでCloneを実装する。

- Derivable

  このトレイトは、全てのフィールドがCloneであれば#[derive]で使用することができます。派生されたCloneの実装は，各フィールドに対してCloneを呼び出します．

  一般的な構造体の場合，#[derive]は一般的なパラメータにバインドされたCloneを追加することで条件付きでCloneを実装します．

~~~rust
  // `derive` implements Clone for Reading<T> when T is Clone.
  #[derive(Clone)]
  struct Reading<T> {
      frequency: T,
  }
~~~

- Cloneを実装するには

  `Copy`である型は、`Clone`の些細な実装を持っていなければなりません。より正式には: `T: Copy`, `x:T`,`y: &T`の場合、`let x = y.clone(); `は`let x = *y;`と等価です。マニュアル実装では、この不変性を維持するように注意しなければなりません。

  例として、関数ポインタを保持する汎用構造体があります。この場合、Cloneの実装は派生できませんが、次のように実装することができます。

~~~rust
  struct Generate<T>(fn() -> T);
  
  impl<T> Copy for Generate<T> {}
  
  impl<T> Clone for Generate<T> {
      fn clone(&self) -> Self {
          *self
      }
  }
~~~

- Cloneトレイトを実装する型

  - 関数ポインタ型
  - 関数定義型
  - すべての要素にCloneな型(Cloneを実装した型)をもつタプル型と配列型
  - 環境に何も補足しない、あるいは、Cloneな型だけを補足したクロージャ型。なお、不変の参照として補足した変数は元の型が何であれCloneを実装する。

---

### [core::marker::Copy](https://doc.rust-lang.org/stable/core/marker/trait.Copy.html)

  - Description

    ビットをコピーするだけで値が複製される型。
    デフォルトでは、変数バインディングは`move semantics`を持っています。言い換えれば

~~~rust
  #[derive(Debug)]
struct Foo;

let x = Foo;

let y = x;

// `x` has moved into `y`, and so cannot be used

// println!("{:?}", x); // error: use of moved value
~~~


  しかし、型がCopyを実装している場合は、代わりに'copy semantics'を持つことになります。

~~~rust
  // We can derive a `Copy` implementation. `Clone` is also required, as it's
// a supertrait of `Copy`.
#[derive(Debug, Copy, Clone)]
struct Foo;

let x = Foo;

let y = x;

// `y` is a copy of `x`

println!("{:?}", x); // A-OK!
~~~

  これら2つの例では、唯一の違いは、代入後にxへのアクセスが許可されているかどうかだけであることに注意することが重要です。この2つの例では、コピーと移動の両方がメモリ内にビットがコピーされる結果になることがありますが、これは時々最適化されています。

  - コピーを実装するには
    型にコピーを実装するには2つの方法があります。最も単純なのは`derive`を使用することです。

 ~~~rust
   #[derive(Copy, Clone)]
struct MyStruct;
 ~~~

  コピーとクローンを手動で実装することもできます。

~~~rust 
  struct MyStruct;

impl Copy for MyStruct { }

impl Clone for MyStruct {
    fn clone(&self) -> MyStruct {
        *self
    }
}
~~~

  この2つの間には小さな違いがあります: `derive`戦略では型のパラメータにも`Copy`が適用されますが、これは必ずしも望ましいものではありません。


  - コピーとクローンの違い
    コピーは暗黙のうちに行われ、例えば、代入`y = x`の一部として行われます。コピーの動作はオーバーロード可能ではありません。
    クローンは明示的なアクションであり、`x.clone()`です。Cloneの実装は、値を安全に複製するために必要な型固有の動作を提供することができます。例えば、String用のCloneの実装では、ヒープ内の指し示す文字列バッファをコピーする必要があります。`String`の値を単純にビット単位でコピーすると、単にポインタをコピーするだけで、二重解放になってしまいます。この理由から、`String`は`Clone`ではありますが、`Copy`ではありません。
    `Clone`は`Copy`のスーパーtraitなので、`Copy`であるものはすべて`Clone`も実装しなければなりません。ある型が`Copy`の場合、その`Clone`の実装は`*self`を返すだけでよいのです（上の例を参照）。

  - 型がコピーになるのははいつか

    型は、そのコンポーネントのすべてがCopyを実装している場合にCopyを実装できます。例えば、この構造体はCopyにすることができます。

~~~rust
    #[derive(Copy, Clone)]
    struct Point {
       x: i32,
       y: i32,
    }
~~~

    構造体は`Copy`である可能性があり、`i32`は`Copy`であるため、PointはCopyになる資格があります。これに対して、次のように考えてみましょう。

~~~rust
    struct PointList {
        points: Vec<Point>,
    }
~~~

    構造体`PointList`は、`Vec<T>` が `Copy`ではないので、`Copy`を実装できません。`Copy`の実装を導出しようとすると、エラーが発生します。

~~~
    the trait `Copy` may not be implemented for this type; field `points` does not implement `Copy`
~~~

    共有参照`(&T)`も`Copy`なので、型が`Copy`ではない型Tの共有参照を保持していても、型はCopyになることができます。次の構造体を考えてみましょう。これは、上から見ても`Copy`ではない型`PointList`の共有参照を保持しているだけなので、`Copy`を実装することができます。

~~~rust
    #[derive(Copy, Clone)]
    struct PointListWrapper<'a> {
        point_list_ref: &'a PointList,
    }
~~~

- 型がコピーできないのはどんなときか

  一般的に言えば、もしあなたの型がCopyを実装できるのであれば、実装すべきです。しかし、`Copy`の実装は型のパブリック`API`の一部であることを覚えておいてください。将来的に型が非コピーになる可能性がある場合は、`API`の変更を避けるために今は`Copy`の実装を省略するのが賢明かもしれません。

- コピートレイトを実装する型


  - すべてのスカラ型。たとえばbool、char、i32、usize、f64型

  - 不変の参照`&T`型、生ポインタ`*const T`型と`*mut T型

    ※可変の参照`&mut T`はCopyトレイトを実装しないことに注意

  - 関数ポインタ型

  - 関数定義型

  - すべての要素にCopyな型(Copyを実装した型)をもつタプル型と配列型

  - 環境に何も補足しない、あるいは、Copyな型だけを補足したクロージャ型。なお、不変の参照として補足した変数は元の型が何であれCopyを実装する。一方、可変の参照として補足した場合はCopyを実装しない

  - すべての要素がCopyな型を持つ`Option<T>`と`Result<T,E>`型

---

### CopyトレイトとCloneトレイトの違い

CopyトレイトとCloneトレイトの違いを以下に示す

| トレイト | コピーの実行                                                 | コピーの処理内容                                             | コピーの実行時コスト             |
| -------- | ------------------------------------------------------------ | ------------------------------------------------------------ | -------------------------------- |
| Copy     | 暗黙的。所有権がムーブする場面で、ムーブの代わりにコピーされる | 単純なバイトレベルのコピー。ロジックのカスタマイズはできない | 低い                             |
| Clone    | 明示的。cloneメソッドによりコピーされる                      | シンプルなロジックから複雑なロジックまで自由に実装できる     | 低いか高いかは処理内容と値に依存 |

---

### ムーブセマンティクス

- 所有権のムーブを伴う操作
  - パターンマッチ(match式だけでなく、letによる変数の束縛も含む)
  - 関数呼びだし
  - 関数やブロックからのリターン
  - コンストラクタ
  - moveクロージャ

---

### 関数ポインタ

- Description

  関数ポインタは、既に定義した関数を渡したい時に有用です。

  関数ポインタで行うと、関数を引数として他の関数に渡して使用できます。関数は、型`fn`(小文字のfです)に型強制されます。 `Fn`クロージャトレイトと混同すべきではありません。`fn`型は、*関数ポインタ*と呼ばれます。 引数が関数ポインタであると指定する記法は、クロージャのものと似ています。

~~~rust
  fn add_one(x: i32) -> i32 {
      x + 1
  }
  
  fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
      f(arg) + f(arg)
  }
  
  fn main() {
      let answer = do_twice(add_one, 5);
  
      // 答えは{}
      println!("The answer is: {}", answer);
  }
~~~

---

### スマートポインタ比較表

| ポインタの種類   | 効率  | ライフタイム | 共有 | 書き換え | スレッドセーフ |
| ---------------- | ----- | ------------ | ---- | -------- | -------------- |
| `&mut T`         | ★★★★★ | 短命         | ×    | ○        | △              |
| `&T`             | ★★★★★ | 短命         | ○    | ×        | △              |
| `&RefCell<T>`    | ★★★★☆ | 短命         | ○    | ○        | ×              |
| `&Mutex<T>`      | ★★★☆☆ | 短命         | ○    | ○        | △              |
| `Box<T>`         | ★★★★☆ | `'static`    | ×    | ○        | ○              |
| `Rc<T>`          | ★★★☆☆ | `'static`    | ○    | ×        | ×              |
| `Rc<RefCell<T>>` | ★★☆☆☆ | `'static`    | ○    | ○        | ×              |
| `Arc<T>`         | ★★☆☆☆ | `'static`    | ○    | ×        | ○              |
| `Arc<Mutex<T>>`  | ★☆☆☆☆ | `'static`    | ○    | ○        | ○              |

- 補足

  - 効率

    トレードオフを強調するために、**相対的な効率**を星の個数で表しています。しかし、**効率をことさら気にする必要はありません**。他の多くのプログラミング言語では、Rustでいうところの `Arc<Mutex<T>>` をデフォルトで使っているような状況です。要件にあったものを使うことが大事です。

  - ライフタイム

    「短命」とついているものは基本的に、ある関数のスコープに紐付いた形でしか使えません。特に**慣れないうちは、これらの型を構造体に入れて使うのはやめておいたほうが無難で**しょう。

    例外として、グローバル変数 (`static`, `lazy_static!`) は `&'static` 参照として扱うことができます

  - 共有

    「ポインタ」と聞くと、参照先が共有されている様子を思い浮かべる人が多いと思いますが、所有権の概念のあるRustでは必ずしもそのイメージは当てはまりません。 **`&mut T` と `Box<T>` は共有のための仕組みとしては使えない**ことに留意するといいでしょう。

  - 書き換え

    Rustでは共有されたデータの書き換えは原則としてできません。これをオプトアウトするために `Mutex` や `RefCell` というコンテナを挟む必要があります。

  - スレッドセーフ

    **スレッドセーフかどうかはコンパイラが検査する**ので、あらかじめ注意しながら書く必要はありません。

    "△" とついているものはスレッド安全ではありますが、短命であるため通常の方法で他のスレッドに送ることができません。rayonなどスコープドスレッドを使う処理では安全に使うことができます。

  - Cell, Atomic, RwLock

    表を簡単にするために以下の型は除きました。

    - `Cell<T>` は `RefCell<T>` と似た特性を持ちます。ロック用の領域が省けるかわりに、限定的な操作しかできなくなります。 `T: Copy` であるときに便利です。

    - `AtomicUsize` は `Mutex<usize>` と似た特性を持ちます。一般的に `Atomic*` は対応する `Mutex<*>` とよく似ています。より効率的ですがロックはできません。

    - `RwLock<T>` は `Mutex<T>` と似た特性を持ちますが、読み取り専用ロックをとれるため読み取りが多い場合はより効率的に使うことができます。

    - Option

      表で挙げた型は全て**non-nullなポインタ型**です。ポインタ型を `Option` で囲めば nullable になります。

    - 関数ポインタ

      関数ポインタ型は `fn(i32, &str) -> bool` のような形で書きます。 `*const fn()` と書いてしまうと関数ポインタへのポインタになってしまうので注意が必要です。

      ほとんどのユースケースでは関数ポインタではなくクロージャポインタである `Box<dyn Fn(i32, &str) -> bool>` のほうが適切です。

    - 生ポインタ

      `*const T`, `*mut T` はFFI用、 `NonNull<T>` はRust内でunsafeなコードを書くためのプリミティブとしてよく出てきます。これらを適切に扱うにはより高度な知識が必要なため本記事では省略します。

    - `Box<T>` について

      `Box<T>` はしばしば**Rustの所有権が役に立つ代表例**として挙げられますが、これはある意味誤解のもとです。確かに **`Box<T>` は優れていますが、これはこの型が多機能だからではなく、むしろ単機能だから優れている**と考えられます。

      実際、 `T` を `Box<T>` に変更しても、以下のような影響しかありません(そしてこれらの特徴は他の多くのポインタ型にも共通して成り立ちます)。

      1. サイズが一定 (1ワード～2ワード) になる。
      2. 単純コピーである `Copy` は使えなくなる。 (`Clone` に置き換える必要がある)
      3. `Pin` で包むことができるようになる。

      3はより高度な使い方なので省略します。また2もデータ設計上重要ではないので基本的には1の**「サイズが一定になる」という恩恵のみが重要**ということになります。この1の恩恵が役に立つのは精々以下のようなケースくらいです。

      - トレイトオブジェクト (`dyn Trait`) を包むため。
      - 再帰的なデータを定義するため。
      - 大きすぎる構造体のムーブコストが嵩む場合。

      繰り返しになりますが、**しばしばポインタに期待される「同じ領域を複数の所有者が共有する」という機能を `Box<T>` は持っていません**。このことに注意して必要なスマートポインタを選ぶのが肝要です。

---

### lazy_static

- Description

  遅延評価されるスタティックを宣言するためのマクロです。

  このマクロを使用すると、初期化されるために実行時にコードを必要とするスタティックを持つことができます。これには、`Vec`や`HashMap`のようにヒープの割り当てを必要とするものや、計算される関数呼び出しを必要とするものが含まれます。

- Syntax

```rust
  lazy_static! {
      [pub] static ref NAME_1: TYPE_1 = EXPR_1;
      [pub] static ref NAME_2: TYPE_2 = EXPR_2;
      ...
      [pub] static ref NAME_N: TYPE_N = EXPR_N;
  }
```

  属性（`doc`コメントを含む）にも対応しています。

```rust
  lazy_static! {
      /// This is an example for using doc comment attributes
      static ref EXAMPLE: u8 = 42;
  }
```

- Semantics

  与えられた`static ref NAME: TYPE = EXPR;`の場合、マクロは、`Deref<TYPE>`を実装したユニークな型を生成し、`NAME`という名前の`static`に格納します。(属性は最終的にこの型に添付されます)。

  最初の`Deref`で`EXPR`が評価されて内部に保存され、以降のすべての`Deref`で同じオブジェクトへの参照を返すことができます。これは、初期化でお互いに依存している複数の遅延スタティックがある場合、デッドロックにつながる可能性があることに注意してください。

  遅延初期化を除けば、結果として得られる `static ref `変数は、通常の `static`変数と同じ性質を持っています。

  - これらの変数に含まれるすべての型は`Sync`トレイトを満たす必要があります。
  - 型がデストラクタを持っている場合は、プロセスが終了したときに実行されません。

- Example

```rust
  #[macro_use]
  extern crate lazy_static;
  
  use std::collections::HashMap;
  
  lazy_static! {
      static ref HASHMAP: HashMap<u32, &'static str> = {
          let mut m = HashMap::new();
          m.insert(0, "foo");
          m.insert(1, "bar");
          m.insert(2, "baz");
          m
      };
      static ref COUNT: usize = HASHMAP.len();
      static ref NUMBER: u32 = times_two(21);
  }
  
  fn times_two(n: u32) -> u32 { n * 2 }
  
  fn main() {
      println!("The map has {} entries.", *COUNT);
      println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
      println!("A expensive calculation on a static results in: {}.", *NUMBER);
  }
```

- implementation details

  `Deref`の実装では、アクセスごとにアトミックなチェックで守られた隠れた静的変数を使用しています。

- Cargo features

  このクレートは1つのcargo機能を備えています。

  - `spin_no_std`:これにより、スタンドアロンの`spin`クレートに依存することで、この`crate`を`std`のない環境で使用できるようになります。



---

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


---

### std::mem

- Description

  メモリを扱うための基本的な関数です。

  このモジュールには、型のサイズや配置の問い合わせ、メモリの初期化や操作のための関数が含まれています。

---

#### std::mem::size_of

- Description

  型のサイズをバイト単位で返します。

  具体的には、そのアイテムタイプを持つ配列の連続する要素間のオフセットをバイト単位で表します（アライメントパディングを含む）。したがって、任意の型Tと長さnに対して、`[T; n]`は`n * size_of::<T>()`のサイズを持ちます。

  一般に、型のサイズはコンパイル時に安定しませんが、プリミティブなどの特定の型は安定します。

  次の表は、プリミティブのサイズを示しています。

  | Type | size_of::<Type>() |
  | :--- | :---------------- |
  | ()   | 0                 |
  | bool | 1                 |
  | u8   | 1                 |
  | u16  | 2                 |
  | u32  | 4                 |
  | u64  | 8                 |
  | u128 | 16                |
  | i8   | 1                 |
  | i16  | 2                 |
  | i32  | 4                 |
  | i64  | 8                 |
  | i128 | 16                |
  | f32  | 4                 |
  | f64  | 8                 |
  | char | 4                 |

  さらに、`usize`と`isize`は同じサイズです。

  型`*const T`, `&T`, `Box<T>`, `Option<&T>`,` Option<Box<T>> `はすべて同じサイズを持っています。`T`が`Sized`の場合、それらの型はすべて`usize`と同じサイズを持ちます。

  ポインタの`mutability`はそのサイズを変えません。そのため、`&T`と`&mut T`は同じサイズになります。同様に、`*const T`と`*mut T`も同じです。

- Size of `#[repr(C)]` items

  アイテムのC表現は、レイアウトが定義されています。このレイアウトでは、すべてのフィールドのサイズが安定していれば、アイテムのサイズも安定しています。

- Size of Structs

  構造体の場合は、以下のアルゴリズムでサイズを決定します。

  構造体の各フィールドを宣言順に並べる。

  - フィールドのサイズを追加する。

  - 現在のサイズを切り上げて、次のフィールドのアラインメントの最も近い倍数にします。

  最後に、構造体のサイズをそのアラインメントの最も近い倍数に丸めます。構造体のアラインメントは、通常、構造体のすべてのフィールドのうち最大のアラインメントとなります。

  Cとは異なり、ゼロサイズの構造体は1バイトに切り上げられません。
  
- Size of Enums

  判別子以外のデータを持たない列挙型は、コンパイルされたプラットフォームのC列挙型と同じサイズになります。

- Size of Unions

  共用体のサイズは、その最大のフィールドのサイズです。

  C言語とは異なり、サイズがゼロの共用体は1バイトに切り上げられません。

- Example

```rust
  use std::mem;
  
  // Some primitives
  assert_eq!(4, mem::size_of::<i32>());
  assert_eq!(8, mem::size_of::<f64>());
  assert_eq!(0, mem::size_of::<()>());
  
  // Some arrays
  assert_eq!(8, mem::size_of::<[i32; 2]>());
  assert_eq!(12, mem::size_of::<[i32; 3]>());
  assert_eq!(0, mem::size_of::<[i32; 0]>());
  
  
  // Pointer size equality
  assert_eq!(mem::size_of::<&i32>(), mem::size_of::<*const i32>());
  assert_eq!(mem::size_of::<&i32>(), mem::size_of::<Box<i32>>());
  assert_eq!(mem::size_of::<&i32>(), mem::size_of::<Option<&i32>>());
  assert_eq!(mem::size_of::<Box<i32>>(), mem::size_of::<Option<Box<i32>>>());
```

  Using `#[repr(C)]`.

```rust
  use std::mem;
  
  #[repr(C)]
  struct FieldStruct {
      first: u8,
      second: u16,
      third: u8
  }
  
  // The size of the first field is 1, so add 1 to the size. Size is 1.
  // The alignment of the second field is 2, so add 1 to the size for padding. Size is 2.
  // The size of the second field is 2, so add 2 to the size. Size is 4.
  // The alignment of the third field is 1, so add 0 to the size for padding. Size is 4.
  // The size of the third field is 1, so add 1 to the size. Size is 5.
  // Finally, the alignment of the struct is 2 (because the largest alignment amongst its
  // fields is 2), so add 1 to the size for padding. Size is 6.
  assert_eq!(6, mem::size_of::<FieldStruct>());
  
  #[repr(C)]
  struct TupleStruct(u8, u16, u8);
  
  // Tuple structs follow the same rules.
  assert_eq!(6, mem::size_of::<TupleStruct>());
  
  // Note that reordering the fields can lower the size. We can remove both padding bytes
  // by putting `third` before `second`.
  #[repr(C)]
  struct FieldStructOptimized {
      first: u8,
      third: u8,
      second: u16
  }
  
  assert_eq!(4, mem::size_of::<FieldStructOptimized>());
  
  // Union size is the size of the largest field.
  #[repr(C)]
  union ExampleUnion {
      smaller: u8,
      larger: u16
  }
  
  assert_eq!(2, mem::size_of::<ExampleUnion>());
```



---

#### std::mem::size_of_val

- Description

  指し示された値のサイズをバイト単位で返します。

  これは通常`size_of::<T>()`と同じです。しかし、スライス`[T]`やトレイトオブジェクトなど、`T`が静的に知られているサイズを持たない場合は、 `size_of_val`を使って動的に知られているサイズを得ることができます。

- Example

```rust
  use std::mem;
  
  assert_eq!(4, mem::size_of_val(&5i32));
  
  let x: [u8; 13] = [0; 13];
  let y: &[u8] = &x;
  assert_eq!(13, mem::size_of_val(y));
```

---

#### std::mem::size_of_val_raw

- Description

  指し示された値のサイズをバイト単位で返します。

  これは通常`size_of::<T>()`と同じです。しかし、スライス`T`やトレイトオブジェクトなど、`T`が静的に知られているサイズを持たない場合は、 `size_of_val_raw`を使って動的に知られているサイズを得ることができます。

- Safety

  この関数は、以下の条件が成立する場合にのみ、安全に呼び出すことができます。

  - `T`が`Sized`の場合、この関数は常に安全に呼び出すことができます。
  - もし、`T`が`Sized`出ない場合・・・
    - スライスの場合、スライスのテールの長さは初期化された整数でなければならず、値全体のサイズ（動的なテールの長さ＋静的なサイズのプレフィックス）は`isize`に収まらなければなりません。
    - トレイトオブジェクトの場合、ポインタの`vtable`部分は、サイズ変更の強制によって得られた有効な`vtable`を指していなければならず、値全体のサイズ（動的なテールの長さ＋静的サイズのプレフィックス）が`isize`に収まっていなければなりません
    - （不安定な）外部型である場合，この関数は常に安全に呼び出すことができますが，外部型のレイアウトがわからないため，パニックを起こしたり，誤った値を返したりする可能性があります。これは、`extern`型の末尾を持つ型への参照に対する`size_of_val`と同じ動作です。
    - そうでない場合は、保守的にこの関数を呼び出すことはできません。

- Example

```rust
  #![feature(layout_for_ptr)]
  use std::mem;
  
  assert_eq!(4, mem::size_of_val(&5i32));
  
  let x: [u8; 13] = [0; 13];
  let y: &[u8] = &x;
  assert_eq!(13, unsafe { mem::size_of_val_raw(y) });
```



---

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

  

---

### [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)

- Description

  エラーは、エラー値に対する基本的な期待を表すトレイトであり、`Result<T, E>`の`E`型の値である。エラーは、`Display`と`Debug`のトレイトを通して自分自身を記述しなければならず、原因の連鎖情報を提供することもあります。

  `Error::source()`は、エラーが「抽象化の境界」を越える場合に一般的に使用されます。あるモジュールが、下位モジュールのエラーに起因するエラーを報告しなければならない場合、`Error::source()`を介してそのエラーにアクセスすることを許可することができます。これにより、上位モジュールが独自のエラーを提供することが可能になり、同時にソースチェーンを介したデバッグのために実装の一部を明らかにすることができます。


---

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



---

### std::covert

- Description

  型間の変換のためのトレイト。

  このモジュールのトレイトは、ある型から別の型への変換方法を提供します。それぞれのトレイトは異なる目的を果たします。

  - 参照から参照への変換を安価に行うための`AsRef`トレイトを実装する。
  - `AsMut`トレイトを実装して、MutableからMutableへの変換を安価に行う。
    値から値への変換を行う`From`を実装する。
  - 現在のクレートの外にある型への値から値への変換を消費するために`Into` トレイトを実装する。
  - `TryFrom`と`TryInto`のトレイトは`From`と`Into`のように動作しますが、変換が失敗する可能性がある場合に実装する必要があります。

このモジュールのトレイトは、複数の型の引数をサポートするような汎用関数のトレイト境界としてよく使われます。例については、各トレイトのドキュメントを参照してください。

  ライブラリ作者としては、`Into<U>`や`TryInto<U>`よりも`From<T>`や`TryFrom<T>`の実装を常に好むべきです。`From`や`TryFrom`はより柔軟性が高く、標準ライブラリに包括的に実装されているおかげで、同等の`Into`や`TryInto`の実装が無料で提供されているからです。Rust 1.41 より前のバージョンをターゲットにしている場合、現在のクレートの外にある型に変換するときに、`Into`または`TryInto`を直接実装する必要があるかもしれません。

---

#### std::convert::Into

- Description

  入力値を消費する値から値への変換のこと。`From`の反対です。

  `Into`の実装を避け、代わりに`From`を実装すべきである。`From`を実装すると、標準ライブラリのブランケット実装のおかげで、自動的に`Into`の実装が提供されます。

  ジェネリック関数に`trait boundary`を指定する場合、`Into`のみを実装した型も使用できるようにするため、`From`よりも`Into`の使用を推奨する。

  注意：このトレイトは失敗してはいけません。変換に失敗する可能性がある場合は、`TryInto`を使用してください。

- 汎用的な実装

  - `From<T> for U`は`Into<U> for T`を意味します。
  - `Into`は反射的であり、`Into<T> for T`が実装されていることを意味します。

- 古いバージョンのRustでの外部型への変換のためのIntoの実装

  Rust 1.41以前では、目的の型が現在のクレートに含まれていない場合、`From`を直接実装することはできませんでした。例えば、このコードを見てみましょう。

```rust
  struct Wrapper<T>(Vec<T>);
  impl<T> From<Wrapper<T>> for Vec<T> {
      fn from(w: Wrapper<T>) -> Vec<T> {
          w.0
      }
  }
```

  これは古いバージョンの言語ではコンパイルできません。これを回避するために、Intoを直接実装することができます。

```rust
  struct Wrapper<T>(Vec<T>);
  impl<T> Into<Vec<T>> for Wrapper<T> {
      fn into(self) -> Vec<T> {
          self.0
      }
  }
```

  重要なのは、`Into`は`From`の実装を提供していないということです（`From`が`Into`を実装するように）。したがって、常に`From`の実装を試み、`From`が実装できない場合には`Into`にフォールバックする必要があります。

- Example

  `String`は`Into<Vec<u8>>`を実装しています。

  指定された型`T`に変換可能なすべての引数を取るジェネリック関数が欲しいことを表現するために，`Into`<T>の`trait bound`を使うことができます。例えば 関数`is_hello`は`Vec<u8>`に変換可能なすべての引数を取ります。

```rust
  fn is_hello<T: Into<Vec<u8>>>(s: T) {
     let bytes = b"hello".to_vec();
     assert_eq!(bytes, s.into());
  }
  
  let s = "hello".to_string();
  is_hello(s);
```


---

#### std::convert::TryInto

- Description

  selfを消費する変換の試みで、高価であるかどうかはわかりません。

  ライブラリの作者は、通常、このトレイトを直接実装すべきではなく、[`TryFrom`](https://doc.rust-lang.org/stable/std/convert/trait.TryFrom.html)トレイトの実装を好むべきです。`TryFrom`トレイトは、より柔軟性があり、標準ライブラリの包括的な実装のおかげで、同等の`TryInto`の実装を無料で提供しています。これについての詳細は、[`Into`](https://doc.rust-lang.org/stable/std/convert/trait.Into.html)のドキュメントを参照してください。

- Implementing `TryInto`

  これは`Into`の実装と同じ制約と理由があります。詳細はこちらをご覧ください。


---

#### std::convert::From

- Descrition

  入力値を消費しながら値から値への変換を行う際に使用します。これは`Into`の逆数です。

  標準ライブラリのブランケット実装のおかげで、`From`を実装することで自動的にIntoの実装が提供されるため、常に`Into`よりも`From`を実装することを好むべきです。

  Rust 1.41以前のバージョンを対象とし、現在のクレート外の型に変換する場合のみ`Into`を実装してください。以前のバージョンでは、Rustの孤児化ルールのため、これらのタイプの変換を行うことができませんでした。詳細は`Into`を参照してください。

  一般的な関数でトレイト境界を指定する場合は、`From`よりも`Into`を使用することをお勧めします。この方法では、`Into`を直接実装した型も引数として使用できます。

  また、エラー処理を行う際にも`From`は非常に便利です。失敗する可能性のある関数を構築する場合、戻り値の型は一般的に `Result<T, E>`の形式になります。`From`は、関数が複数のエラー型をカプセル化した単一のエラー型を返すことを可能にするトレイトことで、エラー処理を単純化します。詳細については、「例」のセクションや書籍を参照してください。

  注意：このトレイトは失敗してはいけません。変換に失敗する可能性がある場合は、`TryFrom`を使用してください。

- Generic Implementation

  - `U`の`From<T>`は`T`の`Into<U>`を意味します。
  - `From`は反射的であり、`T`の`From<T>`が実装されていることを意味します。

- Example

  `String`は`From<&str>`を実装しています。

  `str`から`String`への明示的な変換は以下のように行われます。

~~~rust
  let string = "hello".to_string();
  let other_string = String::from("hello");
  
  assert_eq!(string, other_string);
~~~

  エラー処理を行う際に、独自のエラー型のために `From`を実装すると便利なことがよくあります。基礎となるエラー型を、基礎となるエラー型をカプセル化した独自のカスタムエラー型に変換することで、基礎となる原因に関する情報を失うことなく、単一のエラー型を返すことができます。演算子は、`From`を実装する際に自動的に提供される`Into<CliError>::into`を呼び出すことで、基礎となるエラー型を独自のエラー型に自動的に変換します。コンパイラは、`Into`のどの実装が使用されるべきかを推測します。

~~~rust
  use std::fs;
  use std::io;
  use std::num;
  
  enum CliError {
      IoError(io::Error),
      ParseError(num::ParseIntError),
  }
  
  impl From<io::Error> for CliError {
      fn from(error: io::Error) -> Self {
          CliError::IoError(error)
      }
  }
  
  impl From<num::ParseIntError> for CliError {
      fn from(error: num::ParseIntError) -> Self {
          CliError::ParseError(error)
      }
  }
  
  fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
      let mut contents = fs::read_to_string(&file_name)?;
      let num: i32 = contents.trim().parse()?;
      Ok(num)
  }
~~~




---

#### std::convert::TryFrom

- Description

  シンプルで安全な型変換ですが、ある状況下では制御された方法で失敗することがあります。これは`TryInto`の逆数です。

  これは、些細なことで成功するかもしれないが、特別な処理を必要とするかもしれない型変換を行っているときに便利です。例えば、、`From`トレイトを使って`i64`を`i32`に変換する方法はありません。なぜなら、`i64`には`i32`が表現できない値が含まれている可能性があり、変換によってデータが失われるからです。これには、`i64`を`i32`に切り詰める（基本的には`i64`の値を`i32::MAX`にモジュロして与える）か、単にi32::MAXを返すか、その他の方法で対処することになります。`From`トレイトは完全な変換を目的としているので、`TryFrom`トレイトは型変換がうまくいかない場合にプログラマに知らせ、それをどう処理するかを決めさせます。

- Generic Implementations

  - `U`の`TryFrom<T>`は`T`の`TryInto<U>`を意味する。
  - `try_from`は反射的で、`T`に対する`TryFrom<T>`が実装されていて、失敗しないことを意味しています -- `T`型の値に対して`T::try_from()`を呼び出した場合の関連するエラー型は`Infallible`です。この`!`型が安定したとき、`Infallible`と`!`は同等となるでしょう

  `TryFrom<T>`は以下のように実装できます。

```rust
  use std::convert::TryFrom;
  
  struct GreaterThanZero(i32);
  
  impl TryFrom<i32> for GreaterThanZero {
      type Error = &'static str;
  
      fn try_from(value: i32) -> Result<Self, Self::Error> {
          if value <= 0 {
              Err("GreaterThanZero only accepts value superior than zero!")
          } else {
              Ok(GreaterThanZero(value))
          }
      }
  }
```

- Example

  As described, [`i32`](https://doc.rust-lang.org/stable/std/primitive.i32.html) implements `TryFrom<`[`i64`](https://doc.rust-lang.org/stable/std/primitive.i64.html)`>`:

```rust
  use std::convert::TryFrom;
  
  let big_number = 1_000_000_000_000i64;
  // Silently truncates `big_number`, requires detecting
  // and handling the truncation after the fact.
  let smaller_number = big_number as i32;
  assert_eq!(smaller_number, -727379968);
  
  // Returns an error because `big_number` is too big to
  // fit in an `i32`.
  let try_smaller_number = i32::try_from(big_number);
  assert!(try_smaller_number.is_err());
  
  // Returns `Ok(3)`.
  let try_successful_smaller_number = i32::try_from(3);
  assert!(try_successful_smaller_number.is_ok());
```




---

#### std::convert::AsRef

  - Description

    簡単な参照間変換を行う。

    このトレイトは、可変参照間の変換に使用される`FnMut`に似ている。

    もし、高度な変換を行う必要がある場合は、`From`を`&T型`で実装するか、カスタム関数を実装するほうがよい。

    `AsRef`は、参照と同じシグネチャを持っていますが、いくつか異なる点がある。

     - `AsRef`とは異なり、参照は任意のTに対してブランケット実装(トレイト境界を満たすあらゆる型にトレイトを実装すること)を持っており、参照または値のどちらかを受け取るために使用できる

     - 参照では、参照した値の`Hash`、`Eq`、`Ord`が同等であることが要求される

     - 構造体の単一フィールドのみを借用したい場合は`Asref`を実施できますが、参照は実装できない。

    

    Note:このトレイトは失敗することができない。変換に失敗する可能性がある場合は、`Option<T>`または`Result<T, E>`を返す専用のメソッドを使用すること。

   - Generic Implementations

     `AsRef`は、内部の型が参照または変異可能な参照である場合に自動参照を行う (例: `foo.as_ref()`は、`foo`が`&mut Foo`または`&&mut Foo`の型を持っていても同じように動作する)。

  - Example

    トレイト境界を使うと、指定された型`T`に変換できる限り、異なる型の引数を受け入れることができます。

    例えば`AsRef<str>`を受け取るジェネリック関数を作ることで、`&str`に変換できるすべての参照を引数として受け入れたいことを表現しています。`String`と`&str`はどちらも`AsRef<str>`を実装しているので、どちらも入力引数として受け入れることができます。

~~~rust
fn is_hello<T: AsRef<str>>(s: T) {
   assert_eq!("hello", s.as_ref());
}

let s = "hello";
is_hello(s);

let s = "hello".to_string();
is_hello(s);
~~~



---

#### std::convert::Infallible

- Description

  絶対に起こりえないエラーのためのエラータイプです。

  この列挙型にはバリアントがないため、この型の値が実際に存在することはありません。これは、結果が常に`Ok `であることを示すために、`Result`を使用してエラー タイプをパラメータ化する汎用 API に役立ちます。

  例えば、`TryFrom`トレイト（Resultを返す変換）には、逆の`Into`実装が存在するすべての型に対する包括的な実装があります。

```rust
  impl<T, U> TryFrom<U> for T where U: Into<T> {
      type Error = Infallible;
  
      fn try_from(value: U) -> Result<Self, Infallible> {
          Ok(U::into(value))  // Never returns `Err`
      }
  }
```



---

### std::num::ParseIntError

- Description

  整数を解析する際に返される可能性のあるエラーです。

  このエラーは、`i8::from_str_radix`などのプリミティブな整数型の`from_str_radix()`関数のエラータイプとして使用されます。


---

### std::mem

- Descriptio

  メモリを扱うための基本的な関数
  型のサイズや配列の問い合わせ、メモリの初期化や操作を行うための関数が含まれています。



---

#### std::mem::discriminant

- Description

  vのenum variantを一意に識別する値を返します。

  `T`がenumでない場合、この関数を呼び出しても未定義の動作にはなりませんが、戻り値は不定です。

- Stability

  enumの定義が変更された場合、enumのvariantの判別式は変更される可能性があります。あるバリアントの判別式は、同じコンパイラでコンパイルしても変わりません。

- Example

  これは、実際のデータを無視して、データを運ぶenumを比較するために使用することができます。

```rust
  use std::mem;
  
  enum Foo { A(&'static str), B(i32), C(i32) }
  
  assert_eq!(mem::discriminant(&Foo::A("bar")), mem::discriminant(&Foo::A("baz")));
  assert_eq!(mem::discriminant(&Foo::B(1)), mem::discriminant(&Foo::B(2)));
  assert_ne!(mem::discriminant(&Foo::B(3)), mem::discriminant(&Foo::C(3)));
```

---

#### std::mem::replace

- Description

  `src`を参照されている`dest`に移動させ、前の`dest`の値を返します。

  どちらの値も破棄しません。

  - 2つの変数の値を入れ替えたい場合は、[`swap`](https://doc.rust-lang.org/std/mem/fn.swap.html)を参照してください。
  - デフォルト値に置き換えたい場合は、[`take`](https://doc.rust-lang.org/std/mem/fn.take.html)をご覧ください。

- Example

```rust
  use std::mem;
  
  let mut v: Vec<i32> = vec![1, 2];
  
  let old_v = mem::replace(&mut v, vec![3, 4, 5]);
  assert_eq!(vec![1, 2], old_v);
  assert_eq!(vec![3, 4, 5], v);
```

  `replace`は、構造体のフィールドを別の値に置き換えて消費することができます。置換を行わないと、以下のような問題が発生します。

```rust
  struct Buffer<T> { buf: Vec<T> }
  
  impl<T> Buffer<T> {
      fn replace_index(&mut self, i: usize, v: T) -> T {
          // error: cannot move out of dereference of `&mut`-pointer
          let t = self.buf[i];
          self.buf[i] = v;
          t
      }
  }
```

  `T`は必ずしも`Clone`を実装しているわけではないので、ムーブを避けるために`self.buf[i]`を`clone`することもできないことに注意してください。しかし、`replace`を使えば、そのインデックスにある元の値を`self`から切り離し、それを返すことができます。

```rust
  use std::mem;
  
  impl<T> Buffer<T> {
      fn replace_index(&mut self, i: usize, v: T) -> T {
          mem::replace(&mut self.buf[i], v)
      }
  }
  
  let mut buffer = Buffer { buf: vec![0, 1] };
  assert_eq!(buffer.buf[0], 0);
  
  assert_eq!(buffer.replace_index(0, 2), 0);
  assert_eq!(buffer.buf[0], 2);
```

  

---

### std::io

- Description

  I/Oのコア機能のためのトレイト、ヘルパー、型定義です。

  `std::io`モジュールには、入出力を行う際に必要となる一般的なものが数多く含まれています。このモジュールの最も中心的な部分は、`Read`と`Write`のトレイトであり、入出力を読み書きするための最も一般的なインタフェースを提供しています。

---

#### std::io::BufReader<R>

  - Description

    `BufReader<R>`構造体は、任意のReaderにバッファを追加する。

    `Read`インスタンスを直接操作するのは非常に非効率である。例えば、`TCPStream`で読み取りを呼び出す度にシステムコールが発生します。`BufReader<R>`は、一度にある程度の量を読み取り、その結果をメモリ内のバッファに保持する。

    `BufReader<R>`が削除されると、そのバッファの内容が破棄する。

    同じストリーム上に複数の`BufReader<R>`のインスタンスを作成すると、データが損失する可能性がある。

    `BufReader::into_inner`で`BufReader`を`unwrap`したあと、基となる`Reader`から読み取りを行うと、データを損失することがある。

  - Example

~~~rust
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    
    fn main() -> std::io::Result<()> {
        let f = File::open("log.txt")?;
        let mut reader = BufReader::new(f);
    
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        println!("First line is {} bytes long", len);
        Ok(())
    }
~~~

  - new関連関数

    デフォルトのバッファ容量を持つ新しい `BufReader<R>`を作成する。デフォルトは現在 8 KB 。

---

#### std::io::Result

- Description

  I/O操作に特化した結果型。

  この型は、エラーが発生する可能性のある操作に対して std::io 全体で広く使用されます。

  この型定義は一般的に、`io::Error`を直接書き出すことを避けるために使用され、それ以外の場合は`Result``への直接のマッピングとなります。

  通常のRustのスタイルでは、型を直接インポートするのが一般的ですが、`Result`のエイリアスは、それらを区別しやすくするために、そうではないことが多いです。Result は一般的に`std::result::Result`であると仮定されているため、このエイリアスのユーザは一般的に、`std::result::Result`のインポートをシャドーイングする代わりに`io::Result`を使用することになります。

- Example

  

~~~rust
  use std::io;
  
  fn get_string() -> io::Result<String> {
      let mut buffer = String::new();
  
      io::stdin().read_line(&mut buffer)?;
  
      Ok(buffer)
  }
~~~


---

#### std::io::stdin

- Description

  現在のプロセスの標準入力への新しいハンドルを構築します。

  返される各ハンドルは、ミューテックスを介してアクセスが同期化された共有グローバルバッファへの参照です。ロックをより明確に制御する必要がある場合は、[Stdin::lock](https://doc.rust-lang.org/stable/std/io/struct.Stdin.html#method.lock)メソッドを参照してください。
  
- Note:Windowsの移植性への配慮

  コンソールで操作する場合、このストリームのWindowsの実装では、UTF-8以外のバイトシーケンスをサポートしていません。有効なUTF-8ではないバイトを読み取ろうとすると、エラーが発生します。

- Example

  Using implicit synchronization:

```rust
  use std::io::{self, Read};
  
  fn main() -> io::Result<()> {
      let mut buffer = String::new();
      io::stdin().read_to_string(&mut buffer)?;
      Ok(())
  }
```

  Using explicit synchronization:

```rust
  use std::io::{self, Read};
  
  fn main() -> io::Result<()> {
      let mut buffer = String::new();
      let stdin = io::stdin();
      let mut handle = stdin.lock();
  
      handle.read_to_string(&mut buffer)?;
      Ok(())
  }
```




---

#### std::io::stdout

- Description

  現在のプロセスの標準出力への新しいハンドルを構築します。

  返される各ハンドルは、ミューテックスを介してアクセスが同期化された共有グローバルバッファへの参照です。ロックをより明確に制御する必要がある場合は、`Stdout::lock`メソッドを参照してください。

- Note:Windowsの移植性への配慮

  コンソールで操作する場合、このストリームのWindowsの実装では、UTF-8以外のバイトシーケンスをサポートしていません。有効なUTF-8でないバイトを書き込もうとすると、エラーが発生します。

- Example

  Using implicit synchronization:

```rust
  use std::io::{self, Write};
  
  fn main() -> io::Result<()> {
      io::stdout().write_all(b"hello world")?;
  
      Ok(())
  }
```

  Using explicit synchronization:

```rust
  use std::io::{self, Write};
  
  fn main() -> io::Result<()> {
      let stdout = io::stdout();
      let mut handle = stdout.lock();
  
      handle.write_all(b"hello world")?;
  
      Ok(())
  }
```



---

#### std::io::Stdout::flush

- Description

  この出力ストリームをフラッシュして、一時的にバッファリングされたコンテンツがすべて目的地に到達するようにします。



---

#### std::io::Read::read

- Description

  このソースからいくつかのバイトを指定されたバッファに引き込み、何バイト読まれたかを返します。

  この関数は、データの待ち受けをブロックするかどうかについては何の保証もしませんが、オブジェクトが読み込みのためにブロックする必要があり、ブロックできない場合は、通常は`Err`返り値を介してその旨を通知します。

  このメソッドの戻り値が Ok(n) である場合、`0 <= n <= buf.len()`であることが保証されなければなりません。ゼロでない`n`の値は、バッファ`buf`がこのソースからの`n`バイトのデータで埋め尽くされたことを示します。`n`が 0 の場合は、2 つのシナリオのうちの 1 つを示します。

---

### std::io::Read::read::read_exact

- Description

  `buf`を埋めるのに必要な正確なバイト数を読み込む。

  この関数は、指定されたバッフ`buf`を完全に満たすのに必要な数のバイトを読み込む。

  この関数が呼ばれたとき、`buf`の内容については何も保証されていないので、実装は`buf`の内容のどのプロパティも真であることに頼ることはできない。実装では、`buf`の内容を読むのではなく、`buf`にデータを書き込むだけにしておくことを推奨する。この問題については、[`read`](https://doc.rust-lang.org/stable/std/io/trait.Read.html#tymethod.read)のドキュメントに詳細な説明があります。

- Errors

  この関数が[`ErrorKind::Interrupted`](https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#variant.Interrupted)の種類のエラーに遭遇した場合、そのエラーは無視され、操作は続行されます。

  この関数がバッファを完全に満たす前に "end of file"に遭遇した場合は、[`ErrorKind::UnexpectedEof`](https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#variant.UnexpectedEof)の種類のエラーを返す。この場合、`buf`の内容は特定されない。

  その他の読み込みエラーが発生した場合、本関数は直ちに戻る。この場合、`buf` の内容は特定されない。

  本関数がエラーを返した場合、何バイト読み込んだかは不明であるが、バッファを完全に満たすのに必要な量以上を読み込むことはない。

- Exmaple

  [`File`](https://doc.rust-lang.org/stable/std/fs/struct.File.html)s implement `Read`:

```rust
  use std::io;
  use std::io::prelude::*;
  use std::fs::File;
  
  fn main() -> io::Result<()> {
      let mut f = File::open("foo.txt")?;
      let mut buffer = [0; 10];
  
      // read exactly 10 bytes
      f.read_exact(&mut buffer)?;
      Ok(())
  }
```




---

#### std::io::Read::read_to_string()

  - Description

    End of Fileまですべてのバイトを読み込みバッファに追加する。

    成功した場合、この関数は読み込んでバッファに追加したバイト数を返却する。

    このストリーム内のデータが有効なUTF-8でない場合、エラーを返却し、バッファに追加されない。

    そのほかの、エラーセマンティクスは[read_to_end](https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_end)を参照のこと。



---

#### std::io::Seek

- Description

  Seekトレイトは、バイトのストリーム内で移動できるカーソルを提供します。

  ストリームは通常、固定サイズであり、終端または現在のオフセットのいずれかに対するシークが可能です。

- Example

  Files implement Seek:

```rust
  use std::io;
  use std::io::prelude::*;
  use std::fs::File;
  use std::io::SeekFrom;
  
  fn main() -> io::Result<()> {
      let mut f = File::open("foo.txt")?;
  
      // move the cursor 42 bytes from the start of the file
      f.seek(SeekFrom::Start(42))?;
      Ok(())
  }
```



---

#### std::io::Seek::seek

- Description

  ストリーム内のオフセット（バイト単位）にシークします。

  ストリームの終端を超えてシークすることは可能ですが、その動作は実装によって定義されます。

  シーク操作が正常に完了した場合、このメソッドは、ストリームの開始点からの新しい位置を返します。この位置は後で [`SeekFrom::Start`](https://doc.rust-lang.org/stable/std/io/enum.SeekFrom.html#variant.Start)で使用できます。

- Errors

  負のオフセットにシークするとエラーになります。

---

#### std::io::SeekFrom

- Description

```rust
  pub enum SeekFrom {
      Start(u64),
      End(i64),
      Current(i64),
  }
```

  I/Oオブジェクト内でシークするために可能なメソッドの列挙。

  これは、[`Seek`](https://doc.rust-lang.org/stable/std/io/trait.Seek.html)トレイトによって使用されます。

- Variants

  - `Start(u64)`

    指定したバイト数のオフセットを設定する

  - `End(i64)`

    オフセットを、このオブジェクトのサイズに指定のバイト数を加えたサイズに設定します。

    オブジェクトの終わりを越えてシークすることは可能ですが、バイト0より前にシークすることはエラーになります。

  - `Current(i64)`

    現在の位置に、指定したバイト数を加えたオフセットを設定します。

    オブジェクトの終わりを越えてシークすることは可能ですが、バイト0より前にシークすることはエラーになります。

---

#### std::io::Write

  - Description

    バイト指向のシンクであるオブジェクトのためのトレイト。

    `Write`トレイト の実装者は`writers`と呼ばれることもあります。

    ライターは `write`と`flush`の 2 つのメソッドで定義されています。

    `write`メソッドは、オブジェクトにデータを書き込もうとし、何バイト書き込まれたかを返します。

    `flush`メソッドは、アダプタや明示的なバッファ自体が、バッファリングされたデータがすべて「真のシンク」に押し出されたことを確認するために便利です。

    ライタは、お互いに互換性があるように設計されています。`std::io`の多くの実装では、`Write`トレイトを実装した型を取り、提供しています。

  - flush

    この出力ストリームをフラッシュし、中間的にバッファリングされたすべてのコンテンツが目的地に到達するようにします。

    - Errors
      I/OエラーやEOFに達しているため、すべてのバイトが書き込めなかった場合はエラーとなります。

---

#### std::io::BufRead::lines

  - Description

    Readerの行のイテレータを返す。

    この関数から返されるイテレータは、io::Result<[String]>のインスタンスを返します。返される各文字列は、最後に改行バイト（0xAバイト）やCRLF（0xD、0xAバイト）は持たない。

---

#### std::io::Write::write_all

- Description

  バッファ全体をこのライターに書き込もうとします。

  このメソッドは、書き込むデータがなくなるか、`ErrorKind::Interrupted`以外の種類のエラーが返されるまで、継続して`write`を呼び出します。このメソッドは、バッファ全体が正常に書き込まれるか、そのようなエラーが発生するまで戻りません。このメソッドから生成された`ErrorKind::Interrupted`以外の種類の最初のエラーが返されます。

  バッファにデータが含まれていない場合、このメソッドは決して`write`を呼び出しません。

- Errors

  この関数は、`ErrorKind::Interrupted`以外の種類のエラーで、`write`が返す最初のエラーを返します。

- Example

```rust
  use std::io::prelude::*;
  use std::fs::File;
  
  fn main() -> std::io::Result<()> {
      let mut buffer = File::create("foo.txt")?;
  
      buffer.write_all(b"some bytes")?;
      Ok(())
  }
```

---

### std::path::Path

  - Discription

    パスのスライス。

    この型はパスを検査するための多くの操作をサポートしている。パスをその構成要素に分割したり(Unixでは/で区切って、Windowsでは/または/で区切って)、ファイル名を抽出したり、パスが絶対パスかどうかを判断したりなど。

    非サイズ型であり、常に 参照や`Box`のようなポインタの後ろで使用されなければならない。

---

#### std::path::Path::new

- Description

  文字列スライスを`Path`スライスとして直接ラップします。

  これはコストのかからない変換です。

---

#### std::path::PathBuf

- Description

  所有権のある、変更可能なパス（Stringに似ている）。

  このタイプには、パスをその場で変更する`push`や`set_extension`などのメソッドがあります。また、`Path`への`Deref`を実装しており、`Path`スライス上のすべてのメソッドは、`PathBuf`の値でも利用可能であることを意味します。

  全体的なアプローチについての詳細は、モジュールのドキュメントに記載されています。

- Example

  `push`を使ってコンポーネントから`PathBuf`を構築することができます。

```rust
  use std::path::PathBuf;
  
  let mut path = PathBuf::new();
  
  path.push(r"C:\");
  path.push("windows");
  path.push("system32");
  
  path.set_extension("dll");
```

  しかし、`push`は動的な状況で使用するのがベストです。これは、すべてのコンポーネントを事前に知っている場合には、より良い方法です。

```rust
  use std::path::PathBuf;
  
  let path: PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
```

  これよりももっと良い方法があります。これらはすべて文字列なので、`From::from`を使うことができます。

```rust
  use std::path::PathBuf;
  
  let path = PathBuf::from(r"C:\windows\system32.dll");
```

  どの方法が一番効果的かは、どのような状況にあるかによって異なります。



---

### std::fs::File

- Description

  ファイルシステム上で開かれているファイルへの参照です。

  ファイルのインスタンスは、それがどのようなオプションで開かれたかに応じて、読み書きすることができます。ファイルには、ファイルに含まれる論理カーソルを内部で変更するための`Seek`も実装されています。

  ファイルは、スコープから外れると自動的にクローズされます。クローズ時に検出されたエラーは、`Drop`の実装では無視されます。これらのエラーを手動で処理する必要がある場合は、`sync_all`メソッドを使用してください。

- Example

  新しいファイルを作成し、そのファイルにバイトを書き込みます（`write()`を使用することもできます）。

```rust
  use std::fs::File;
  use std::io::prelude::*;
  
  fn main() -> std::io::Result<()> {
      let mut file = File::create("foo.txt")?;
      file.write_all(b"Hello, world!")?;
      Ok(())
  }
```

  ファイルの内容を文字列に読み込みます（`read`でも可）。

```rust
  use std::fs::File;
  use std::io::prelude::*;
  
  fn main() -> std::io::Result<()> {
      let mut file = File::open("foo.txt")?;
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      assert_eq!(contents, "Hello, world!");
      Ok(())
  }
```

  バッファ付きの`Reader`でファイルの内容を読む方が効率的な場合があります。これは、`BufReader<R>`で実現できます。

```rust
  use std::fs::File;
  use std::io::BufReader;
  use std::io::prelude::*;
  
  fn main() -> std::io::Result<()> {
      let file = File::open("foo.txt")?;
      let mut buf_reader = BufReader::new(file);
      let mut contents = String::new();
      buf_reader.read_to_string(&mut contents)?;
      assert_eq!(contents, "Hello, world!");
      Ok(())
  }
```

  読み込みと書き込みのメソッドは、`&mut File`を必要としますが、[`Read`](https://doc.rust-lang.org/stable/std/io/trait.Read.html)と [`Write`](https://doc.rust-lang.org/stable/std/io/trait.Write.html)インターフェイスがあるため、`&File`を持つ人は、`&File`を取るメソッドを使ったり、基礎となるOSオブジェクトを取得して、その方法でファイルを変更することができますのでご注意ください。さらに、多くのオペレーティングシステムでは、異なるプロセスによるファイルの同時変更が可能です。また、多くのオペレーティングシステムでは、異なるプロセスによるファイルの同時変更が可能ですので、`&File`を保持することでファイルが変更されないと考えることは避けてください。

---

#### std::fs::File::metadata

- Description

  ファイルのメタデータを取得します。

- Example

```rust
  use std::fs::File;
  
  fn main() -> std::io::Result<()> {
      let mut f = File::open("foo.txt")?;
      let metadata = f.metadata()?;
      Ok(())
  }
```

  

---

#### std::fs::File::open

  - Description

    読み取り専用でファイルを開く。

    この関数は、パスが既に存在しない場合にエラーを返す。

---

#### std::fs::File::create

- Description

  ファイルを書き込み専用で開きます。

  この関数は、ファイルが存在しない場合はファイルを作成し、存在する場合は切り捨てます。

  詳しくは、`OpenOptions::open`関数を参照してください。

---

### std::fs::OpenOptions

- Description

  ファイルがどのように開かれるかを設定するために使用できるオプションとフラグ。

  このBuilderは、ファイルがどのように開かれるか、また、開かれたファイルに対してどのような操作が許可されるかを設定する機能を公開します。`File::open`と`File::create`メソッドは、このBuilderを使ってよく使われるオプションのエイリアスです。

  一般的に言って、オープンオプションを使うときは、最初に `OpenOptions::new`を呼び出し、次に各オプションを設定するためのメソッドへの呼び出しを連鎖させ、次に、開こうとしているファイルのパスを渡して `OpenOptions::open`を呼び出します。これにより、さらに操作できるファイルを内包した`io::Result`が得られます。

- Example

  読み込み用のファイルを開きます。

```rust
  use std::fs::OpenOptions;
  
  let file = OpenOptions::new().read(true).open("foo.txt");
```

  読み書き両用でファイルをオープンしたり、ファイルが存在しない場合は作成したりする。

```rust
  use std::fs::OpenOptions;
  
  let file = OpenOptions::new()
              .read(true)
              .write(true)
              .create(true)
              .open("foo.txt");
```



---

#### std::fs::OpenOptions::new

- Description

  設定可能な空の新しいオプションセットを作成します。

  すべてのオプションは、最初は`false`に設定されています。

- Example

```rust
  use std::fs::OpenOptions;
  
  let mut options = OpenOptions::new();
  let file = options.read(true).open("foo.txt");
```



---

#### std::fs::OpenOptions::read

- Description

  読み込みアクセスのオプションを設定します。

  このオプションが`true`の場合、ファイルを開いたときに読み取り可能であることを示します。

- Example

```rust
  use std::fs::OpenOptions;
  
  let file = OpenOptions::new().read(true).open("foo.txt");
```



---

#### std::fs::OpenOptions::write

- Description

  書き込みアクセスのオプションを設定します。

  このオプションを `true`にすると、ファイルを開いたときに書き込みが可能であることを示します。

  ファイルがすでに存在している場合、書き込みを行うと、その内容が切り捨てられることなく上書きされます。

- Example

```rust
  use std::fs::OpenOptions;
  
  let file = OpenOptions::new().write(true).open("foo.txt");
```



---

#### std::fs::OpenOptions::create

- Description

  新しいファイルを作成するか、既に存在している場合はそれを開くかのオプションを設定します。

  ファイルを作成するためには、`OpenOptions::write`または `OpenOptions::append`アクセスが使用されなければなりません。

- Example

```rust
  use std::fs::OpenOptions;
  
  let file = OpenOptions::new().write(true).create(true).open("foo.txt");
```



---

#### std::fs::OpenOptions::open

- Description

  `self`で指定されたオプションを使って、`path`にあるファイルを開きます。

- Errors

  この関数は、さまざまな状況下でエラーを返します。これらのエラー条件のいくつかを、その`io::ErrorKind`と共にここに示します。`io::ErrorKinds`へのマッピングは、この関数の互換性契約の一部ではありません。特に、その他の種類については、将来、より具体的な種類に変更される可能性があります。

  - `NotFound`: 指定されたファイルが存在せず、createもcreate_newも設定されていません。
  - `PermissionDenied`: ユーザーはファイルの指定されたアクセス権を取得する権限を持っていません。
  - `PermissionDenied`: 指定されたパスのディレクトリ構成要素のひとつを開く権限がユーザーにありません。
  - `AlreadyExists`: create_newが指定され、ファイルはすでに存在しています。
  - `InvalidInput`: オープンオプションの組み合わせが無効（書き込み権限のない切り捨て、アクセスモードが設定されていない、など）。
  - `InvalidInput`: オープンオプションの組み合わせが無効（書き込み権限のない切り捨て、アクセスモードが設定されていない、など）。
  - `Other`: 指定されたファイルパスのディレクトリ構成要素の1つが、実際にはディレクトリではありませんでした。
  - その他のファイルシステムレベルのエラー：フルディスク、読み取り専用のファイルシステムに書き込み権限が要求された、ディスククォータを超えた、開いているファイルが多すぎる、ファイル名が長すぎる、指定されたパスにシンボリックリンクが多すぎる（Unix系システムのみ）、など。

- Example

```rust
  use std::fs::OpenOptions;
  
  let file = OpenOptions::new().read(true).open("foo.txt");
```



---

### std::fs

---

#### std::fs::write

- Descriptiom

  スライスをファイルの内容全体として書き込みます。

  この関数は、ファイルが存在しない場合にはファイルを作成し、存在する場合にはその内容を完全に置き換えます。

  これは、`File::create`や`write_all`を使用してインポートを少なくするための便利な関数です。

  - Example

~~~rust
    use std::fs;
    
    fn main() -> std::io::Result<()> {
        fs::write("foo.txt", b"Lorem ipsum")?;
        fs::write("bar.txt", "dolor sit")?;
        Ok(())
    }
~~~



---

### std::env

---

#### std::env::Args

  - Description

    プロセスの引数に対するイテレータで、各引数の`String`値を返す。
    この構造体は`env::args()`によって作成される。詳細はドキュメントを参照のこと。
    最初の要素は伝統的に実行ファイルのパスですが、任意のテキストを設定することもでき、存在しない場合もある。つまり、このプロパティはセキュリティのために頼るべきではないということである。

---

#### std::env::args

  - Description

    このプログラムが開始されたときの引数（通常はコマンドラインで渡される）を返します。
    
    最初の要素は伝統的には実行ファイルのパスですが、任意のテキストに設定することができ、存在しない可能性もあります。つまり、このプロパティをセキュリティ目的で信頼してはいけないということです。
    
    Unixシステムでは、シェルは通常、引用符で囲まれていない引数をグロブパターン（`*`や`?`など）で展開します。Windowsではこのような処理は行われず、そのような引数はそのまま渡されます。
    
    glibc Linuxシステムでは、引数は、関数を`.init_array`に置くことで取得されます。glibcは、非標準の拡張として、`argc`、`argv`、および`envp`を`.init_array`の関数に渡します。これにより、`cdylib`や`staticlib`であっても、macOSやWindowsのように`std::env::args`が動作するようになります。
    
- Panics

  返されたイテレータは、プロセスへの引数が有効なUnicodeでない場合、反復中にパニックを起こします。これが望ましくない場合は、代わりに`args_os`関数を使用してください。

- Example

```rust
  use std::env;
  
  // Prints each argument on a separate line
  for argument in env::args() {
      println!("{}", argument);
  }
```

  

---

#### std::env::set_current_dir

- Description

  現在の作業ディレクトリを、指定したパスに変更します。

  操作に失敗した場合は、`Err`を返します。

---

#### std::env::var

  - Description

    現在のプロセスから環境変数のキーを取得する。

---

### std::borrow

---

#### std::borrow::ToOwned

- Description

  通常は、クローンを作成することによって、借用したデータから所有データを作成します。

---

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

---

### std::default

---

#### std::default::Default

  - Description
    型に有用なデフォルト値を与えるためのトレイト。

    ある種のデフォルト値にフォールバックしたい場合がありますが、それが何であるかは特に気にしません。これは、オプションのセットを定義する構造体でよく出てきます。

~~~rust
struct SomeOptions {
    foo: i32,
    bar: f32,
}
~~~

デフォルト値を定義するには、既定値を使用することができます。

~~~rust
#[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}

fn main() {
    let options: SomeOptions = Default::default();
}
~~~

これで、すべてのデフォルト値を取得できます。Rustは様々なプリミティブ型に対して`Default`を実装しています。
特定のオプションをオーバーライドしても、他のデフォルト値を保持したい場合。

~~~rust
fn main() {
    let options = SomeOptions { foo: 42, ..Default::default() };
}
~~~

  - Derivable
    このトレイトは、型のすべてのフィールドが Default を実装している場合に #[derive] を使用することができます。派生された場合、各フィールドの型のデフォルト値が使用されます。

  - デフォルトを実装するには
    `default()`メソッドの実装を提供し、デフォルトとなるべき型の値を返すようにします。

~~~rust
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Self { Kind::A }
}
~~~

  - Example

~~~rust
  #[derive(Default)]
struct SomeOptions {
    foo: i32,
    bar: f32,
}
~~~

---

### std::fmt

---

#### std::fmt::Debug

  - Description
    `?`フォーマット。
    `Debug`プログラマー向けのデバッグコンテキストで出力をフォーマットする必要があります。
    一般的に言って、あなたはただ`derive`の`Debug`実装であるべきです。
     代替フォーマット指定子と一緒に使用すると、`#?`出力はきれいにprintされます。
    フォーマッタの詳細については、モジュールレベルの[ドキュメント](https://doc.rust-lang.org/core/fmt/index.html)を参照してください。
    この特性は、すべてのフィールドが`Debug`を実装している場合、＃[derive]とともに使用できます。 構造体用に派生する場合、構造体の名前、{、各フィールドの名前とデバッグ値のコンマ区切りリスト、}の順に使用します。 列挙型の場合は、バリアントの名前と、該当する場合は（、フィールドのデバッグ値、次に）を使用します。

    ◆実装の導出

~~~rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

assert_eq!(format!("The origin is: {:?}", origin), "The origin is: Point { x: 0, y: 0 }");
~~~

​    ◆手動で実装

~~~rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

let origin = Point { x: 0, y: 0 };

assert_eq!(format!("The origin is: {:?}", origin), "The origin is: Point { x: 0, y: 0 }");
~~~

  - Example
    `Formatter`構造体には、手動での実装を支援するためのヘルパーメソッドがいくつかあります:debug_struct。
    `Debugderive`またはデバッグビルダーAPIを使用した実装で`Formatter`は、代替フラグを使用したプリティプリントがサポートされます`{:#?}`。
- Pretty-printing with `#?`:

~~~rust
＃[ derive（Debug）] 
struct  Point {
     x：i32、
     y：i32、
} let origin = Point { x：0、y：0 }; assert_eq ！（format ！（"原点は：{：＃？}"、origin）、
 "原点は：Point { 
    x：0、
    y：0、
}"）;
~~~

---

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

    - `take()`は、無限イテレータを使って有限にするためによく使われる

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

  

---

### std::vec

---
#### std::vec::IntoIter

- Description

  ベクトルの外に移動するイテレータ。

  この構造体は、`[Vec]`の`into_iter`メソッドによって作成されます（`[IntoIterator] trait`によって提供されます）。

---

#### std::vec::Vec::extend_from_slice

- Description

  スライス内のすべての要素をクローン化し、`Vec`に追加します。

  他のスライスを反復処理し、各要素をクローン化し、この`Vec`に追加します。他のベクトルは順番に巡回されます．

  この関数は`extend`と同じですが、代わりにスライスを扱うように特化されていることに注意してください。Rustが特殊化された場合、この関数はおそらく非推奨になるでしょう（しかし、まだ利用可能です）。

- Example

~~~rust
  let mut vec = vec![1];
  vec.extend_from_slice(&[2, 3, 4]);
  assert_eq!(vec, [1, 2, 3, 4]);
~~~

---

#### std::vec::Vec::dedup

- Description

  [PartialEq](https://doc.rust-lang.org/stable/std/cmp/trait.PartialEq.html)トレイトの実装に従って，ベクトル中の連続した繰り返し要素を削除します。

  `Vec`がソートされている場合は，すべての重複した要素を削除します。

- Example

```rust
  let mut vec = vec![1, 2, 2, 3, 2];
  
  vec.dedup();
  
  assert_eq!(vec, [1, 2, 3, 2]);
```



---

#### std::vec::Vec::with_capacity

- Description

  指定された容量の新しい空の`Vec<T>`を作成します。

  `Vec`は、再割り当てを行わずに正確に容量要素を保持することができます。capacityが0の場合、ベクタは割り当てを行いません。

  返されたベクタは指定された容量を持っていますが、ベクタの長さはゼロになることに注意することが重要です。長さと容量の違いについての説明は、容量と再割り当てを参照してください。



---

#### std::vec::Vec::into_boxed_slice

- Description

  Vectorを`Box<[T]>`に変換します。

  これにより、余った容量がなくなることに注意してください。

- Example

```rust
  let v = vec![1, 2, 3];
  
  let slice = v.into_boxed_slice();
```

  Any excess capacity is removed:

```rust
  let mut vec = Vec::with_capacity(10);
  vec.extend([1, 2, 3].iter().cloned());
  
  assert_eq!(vec.capacity(), 10);
  let slice = vec.into_boxed_slice();
  assert_eq!(slice.into_vec().capacity(), 3);
```




---

#### std::vec::Vec::resize_with

- Description

  `len`が`new_len`と等しくなるように、`Vec`のサイズをその場で変更します。

  `new_len`が`len`よりも大きい場合、`Vec`はその差だけ拡張され、追加された各スロットはクロージャ`f`を呼び出した結果で埋められます。

  `new_len`が`len`よりも小さい場合、`Vec`は単に切り捨てられます。

  このメソッドはクロージャを使って、プッシュのたびに新しい値を生成します。与えられた値のクローンを作りたい場合は、`Vec::resize`を使います。`Default`トレイトを使って値を生成したい場合には、2 番目の引数に`Default::default`を渡します。

- Example

```rust
  let mut vec = vec![1, 2, 3];
  vec.resize_with(5, Default::default);
  assert_eq!(vec, [1, 2, 3, 0, 0]);
  
  let mut vec = vec![];
  let mut p = 1;
  vec.resize_with(4, || { p *= 2; p });
  assert_eq!(vec, [2, 4, 8, 16]);
```



---

#### std::vec::Vec::insert

- Description

  `Vec<T>`の位置indexに要素を挿入し、それ以降のすべての要素を右にシフトします。

- Example

```rust
  let mut vec = vec![1, 2, 3];
  vec.insert(1, 4);
  assert_eq!(vec, [1, 4, 2, 3]);
  vec.insert(4, 5);
  assert_eq!(vec, [1, 4, 2, 3, 5]);
```


---

#### std::vec::Vec::reserve

- Description

  与えられた`Vec<T>`に挿入される少なくとも追加の要素のための容量を確保します。コレクションは、頻繁な再割り当てを避けるために、より多くの容量を確保することができます。`reserve`を呼び出した後の容量は、`self.len() + additional`以上になります。容量がすでに十分な場合は何もしません。

- Panics

  新しい容量が`isize::MAX`バイトを超えるとパニックになります。

- Example

```rust
  let mut vec = vec![1];
  vec.reserve(10);
  assert!(vec.capacity() >= 11);
```


---

#### std::vec::Vec::append

- Description

  他方の要素をすべて自分自身に移動させ、他方を空にします。

- Panics

  ベクタの要素数が`usize`を超えた場合、パニックを起こします。

- Example

```rust
  let mut vec = vec![1, 2, 3];
  let mut vec2 = vec![4, 5, 6];
  vec.append(&mut vec2);
  assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
  assert_eq!(vec2, []);
```



---

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

    

---

### std::process

  - Description

    プロセスを扱うモジュール。

    このモジュールは、主に子プロセスの生成と相互作用に関係していますが、現在のプロセスを終了させるための`abort`と `exit]`も提供している。

---

#### std::process::exit

  - Description

    指定した終了コードで現在のプロセスを終了させる。

    この関数は何も返却せず、現在のプロセスを即座に終了させる。終了コードは基盤となるOSに渡され、別のプロセスで使用できるようになる。

    この関数は何も返却せず、プロセスを終了するので、現在のスタックや他のスレッドのスタック上のデストラクタは実行されないことに注意すること。クリーンなシャットダウンが必要な場合は、実行するデストラクタがなくなった時点でのみこの関数を呼び出すことを検討すること。

---

####  std::process::Command

- Description

  プロセスビルダは、新しいプロセスをどのように生成するかをきめ細かく制御します。

  デフォルトの設定は、`Command::new(program)`を使って生成することができ、programには実行されるプログラムのパスが与えられます。追加のビルダメソッドにより、生成前に設定を変更することができます（例えば、引数を追加するなど）。

```rust
  use std::process::Command;
  
  let output = if cfg!(target_os = "windows") {
      Command::new("cmd")
              .args(&["/C", "echo hello"])
              .output()
              .expect("failed to execute process")
  } else {
      Command::new("sh")
              .arg("-c")
              .arg("echo hello")
              .output()
              .expect("failed to execute process")
  };
  
  let hello = output.stdout;
```

  `Command`は、複数のプロセスを起動するために再利用できます。ビルダー・メソッドは、すぐにプロセスを起動することなく、コマンドを変更します。

```rust
  use std::process::Command;
  
  let mut echo_hello = Command::new("sh");
  echo_hello.arg("-c")
            .arg("echo hello");
  let hello_1 = echo_hello.output().expect("failed to execute process");
  let hello_2 = echo_hello.output().expect("failed to execute process");
```

  同様に、プロセスを起動した後にビルダーメソッドを呼び出し、変更した設定で新しいプロセスを起動することができます。

```rust
  use std::process::Command;
  
  let mut list_dir = Command::new("ls");
  
  // Execute `ls` in the current directory of the program.
  list_dir.status().expect("process failed to execute");
  
  println!();
  
  // Change `ls` to execute in the root directory.
  list_dir.current_dir("/");
  
  // And then execute `ls` again but in the root directory.
  list_dir.status().expect("process failed to execute");
```

- Implementations

```rust
  pub fn new<S: AsRef<OsStr>>(program: S) -> Command
```

  パス program でプログラムを起動するための新しい Command を、以下のデフォルト設定で構築します。

  - プログラムへの引数なし
  - 現在のプロセスの環境を継承します。
  - 現在のプロセスの作業ディレクトリを継承します。
  - `spawn`や`status`のために`stdin/stdout/stderr`を継承するが、`output`のためにパイプを作成する。

  これらのデフォルト値を変更したり、プロセスを設定するためのビルダー・メソッドが用意されています。

  programが絶対パスでない場合、`PATH`は`OS`で定義された方法で検索されます。

  使用される検索パスは、コマンドで`PATH`環境変数を設定することで制御できますが、`Windows`では実装上の制限があります（`#37519`を参照）。

- Example

  Basic usage:

```rust
  use std::process::Command;
  
  Command::new("sh")
          .spawn()
          .expect("sh command failed to start");
```



---

#### std::process::Command::spawn

- Description

  コマンドを子プロセスとして実行し、そのハンドルを返します。

  デフォルトでは、`stdin`、`stdout`、`stderr`が親プロセスから継承されます。

- Example

```rust
  use std::process::Command;
  
  Command::new("ls")
          .spawn()
          .expect("ls command failed to start");
```




---

#### std::process::Command::output

- Description

  子プロセスとしてコマンドを実行し、コマンドの終了を待ち、その出力をすべて収集します。

  デフォルトでは、`stdout`と`stderr`が収集されます（結果の出力にも使用されます）。`stdin`は親プロセスから継承されず、子プロセスが`stdin`ストリームから読み取ろうとすると、ストリームは直ちに閉じられます。

- Example

```rust
  use std::process::Command;
  use std::io::{self, Write};
  let output = Command::new("/bin/cat")
                       .arg("file.txt")
                       .output()
                       .expect("failed to execute process");
  
  println!("status: {}", output.status);
  io::stdout().write_all(&output.stdout).unwrap();
  io::stderr().write_all(&output.stderr).unwrap();
  
  assert!(output.status.success());
```

  

---

#### std::process::Child

- Description

  実行中または終了した子プロセスの表現。

  この構造体は、子プロセスを表現し、管理するために使用されます。子プロセスは、`Command`構造体を介して作成されます。`Command`構造体は、生成プロセスを設定し、ビルダースタイルのインターフェイスを使用してそれ自体を構築することができます。

  子プロセスの`Drop`は実装されていないので、子プロセスが終了したことを確認しないと、子プロセスの`Child`ハンドルがスコープ外に出た後でも、子プロセスは実行され続けます。

  `wait`(または`wait`をラップした他の関数) を呼び出すと、親プロセスは子が実際に終了するまで待ってから続行します。

- Warning

  システムによっては、OSがリソースを解放するために`wait`などの呼び出しが必要な場合があります。終了したのにwaitされていないプロセスは「ゾンビ」として残っています。あまり多くのゾンビを放置すると、グローバルリソース（プロセスIDなど）を使い果たしてしまう可能性があります。

  標準ライブラリは子プロセスを自動的に待ちません（子プロセスがドロップされても待ちません）。そのため、長時間動作するアプリケーションでは、最初に待機せずに`Child`ハンドルをドロップすることは推奨されません。システムによっては、OSがリソースを解放するためにwaitなどの呼び出しが必要な場合があります。終了したのに`wait`されていないプロセスは「ゾンビ」として残っています。あまり多くのゾンビを放置すると、グローバルリソース（プロセスIDなど）を使い果たしてしまう可能性があります。

  標準ライブラリは子プロセスを自動的に待ちません（子プロセスがドロップされても待ちません）。そのため、長時間動作するアプリケーションでは、最初に待機せずに`Child`ハンドルをドロップすることは推奨されません。

- Example

```rust
  use std::process::Command;
  
  let mut child = Command::new("/bin/cat")
                          .arg("file.txt")
                          .spawn()
                          .expect("failed to execute child");
  
  let ecode = child.wait()
                   .expect("failed to wait on child");
  
  assert!(ecode.success());
```

---

### std::string::String

---

#### String::from_utf8_lossy

- Description

  無効な文字を含むバイトのスライスを文字列に変換します。

  文字列はバイト([u8])でできており、バイトのスライス(&[u8])はバイトでできているので、この関数は両者を変換します。ただし、すべてのバイトスライスが有効な文字列であるわけではありません: 文字列は有効なUTF-8である必要があります。この変換の際、`from_utf8_lossy()`は無効な UTF-8 シーケンスを`U+FFFD REPLACEMENT CHARACTER`で置き換えます。

  バイトスライスが有効なUTF-8であることが確実で、変換のオーバーヘッドを発生させたくない場合は、この関数の安全でないバージョンである`from_utf8_unchecked`があります。

  この関数は`Cow<'a, str>`を返します。バイトスライスが無効なUTF-8であれば、置換文字を挿入する必要がありますが、これは文字列のサイズを変えることになるので、Stringが必要になります。しかし、すでに有効なUTF-8であれば、新しい割り当ては必要ありません。この戻り値型は、両方のケースを処理することができます。

---

#### std::string::String::as_str

- Description

  文字列全体を含む文字列スライスを抽出します。

- Example

```rust
  let s = String::from("foo");
  
  assert_eq!("foo", s.as_str());
```


---

#### std::string::String::into_bytes

- Description

  `String`をバイトベクターに変換します。

  これは、`String`を消費するので、その内容をコピーする必要はありません。

- Example

```rust
  let s = String::from("hello");
  let bytes = s.into_bytes();
  
  assert_eq!(&[104, 101, 108, 108, 111][..], &bytes[..]);
```



---

### std::str

---

#### std::str::from_utf8

- Description

  バイトのスライスを文字列のスライスに変換します。

  文字列スライス(`&str`)はバイト(`[u8]`)でできており、バイトスライス(`&[u8]`)はバイトでできているので、この関数は両者を変換します。`from_utf8()`は、バイトが有効なUTF-8であることを確認してから変換を行います。

  バイトスライスが有効な UTF-8 であることが確実で、有効性チェックのオーバーヘッドを発生させたくない場合は、この関数の安全ではないバージョンの`[from_utf8_unchecked]`があります。

  `str`の代わりに`String`が必要な場合は、`String::from_utf8`を検討してください。

  `[u8; N]`をスタック割り当てでき、その`&[u8]`を取ることができるので、この関数はスタック割り当てされた文字列を持つ一つの方法です。以下の例のセクションに例があります。

---

### str

---

#### str::contains

  - Description

    与えられたパターンが、この文字列スライスのサブスライスにマッチした場合に真を返す。
    そうでない場合は false を返す。
    パターンには、`&str`、`char`、文字列のスライス、文字がマッチするかどうかを判定する関数やクロージャを指定することができる。

---

#### str::chars

- Description

  文字列スライスの[`char`](https://doc.rust-lang.org/stable/std/primitive.char.html)に対するイテレータを返します。

  文字列スライスは有効なUTF-8で構成されているので、文字列スライスを[`char`](https://doc.rust-lang.org/stable/std/primitive.char.html)ごとに反復することができます。このメソッドは、そのようなイテレータを返します。

  [`char`](https://doc.rust-lang.org/stable/std/primitive.char.html)は Unicode Scalar Value を表しており、「文字」の概念とは異なることを覚えておく必要があります。実際に必要なのは、書記素クラスタのイテレータかもしれません。この機能はRustの標準ライブラリでは提供されていないので、代わりにcrates.ioをチェックしてください。

- Example

```rust
  let word = "goodbye";
  
  let count = word.chars().count();
  assert_eq!(7, count);
  
  let mut chars = word.chars();
  
  assert_eq!(Some('g'), chars.next());
  assert_eq!(Some('o'), chars.next());
  assert_eq!(Some('o'), chars.next());
  assert_eq!(Some('d'), chars.next());
  assert_eq!(Some('b'), chars.next());
  assert_eq!(Some('y'), chars.next());
  assert_eq!(Some('e'), chars.next());
  
  assert_eq!(None, chars.next());
```



---

#### str::split_whitespace

- Description

  文字列スライスをホワイトスペースで分割します。

  返されるイテレータは、元の文字列スライスのサブスライスで、任意の量のホワイトスペースで区切られた文字列スライスを返します。

  空白」は、Unicode Derived Core Property White_Spaceの条件に従って定義されます。ASCIIホワイトスペースでのみ分割したい場合は、[`split_ascii_whitespace`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.split_ascii_whitespace)を使用してください。

- Example

  Basic Usage:

```rust
  let mut iter = "A few words".split_whitespace();
  
  assert_eq!(Some("A"), iter.next());
  assert_eq!(Some("few"), iter.next());
  assert_eq!(Some("words"), iter.next());
  
  assert_eq!(None, iter.next());
```

  あらゆる種類のホワイトスペースが考慮されます。

```rust
  let mut iter = " Mary   had\ta\u{2009}little  \n\t lamb".split_whitespace();
  assert_eq!(Some("Mary"), iter.next());
  assert_eq!(Some("had"), iter.next());
  assert_eq!(Some("a"), iter.next());
  assert_eq!(Some("little"), iter.next());
  assert_eq!(Some("lamb"), iter.next());
  
  assert_eq!(None, iter.next());
```




---

#### str::lines

  - Description

    各行の文字列を文字列スライスとして、イテレータを返す。

    行は、改行（\ n）または改行によるキャリッジリターン（\ r \ n）のいずれかで終了する。

    最終行終了はオプションである。最終行終了で終わる文字列は、最終行終了のない、そうでなければ同一の文字列と同じ行を返す。



---

#### str::to_lowercase

  - Description

    この文字列スライスの小文字に相当するものを、新しい [String] として返す。
    `Lowercase`は、Unicode Derived Core Property Lowercaseの条項に従って定義される。
    大文字小文字を変更すると複数の文字に展開されてしまう文字があるため、この関数はパラメータをそのまま変更するのではなく、[String]として返す。



---

#### str::parse

- Description

  この文字列スライスを別の型にパースします。

  `parse`は非常に一般的なので、型の推論に問題が生じることがあります。そのため、`parse`は「ターボフィッシュ」として親しまれている構文（`::<>`）を目にする数少ない機会となっています。これは、型推論アルゴリズムが、どの型にパースしようとしているのかを具体的に理解するのに役立ちます。

  `parse`は、`FromStr`トレイトを実装したあらゆる型を解析することができます。

- Errors

  この文字列スライスを目的の型にパースできない場合は`Err`を返します。

- Example

```rust
  let four: u32 = "4".parse().unwrap();
  
  assert_eq!(4, four);
```

  Using the ‘turbofish’ instead of annotating `four`:

```rust
  let four = "4".parse::<u32>();
  
  assert_eq!(Ok(4), four);
```

  Failing to parse:

```rust
  let nope = "j".parse::<u32>();
  
  assert!(nope.is_err());
```


---

#### str::find

- Description

  この文字列スライスのパターンにマッチする最初の文字のバイトインデックスを返します。

  パターンにマッチしない場合は`None`を返します。

  パターンは、`&str`、`char`、`chars`のスライス、または、文字がマッチするかどうかを判断する関数やクロージャです。

- Example

  Simple patterns:

```rust
  let s = "Löwe 老虎 Léopard Gepardi";
  
  assert_eq!(s.find('L'), Some(0));
  assert_eq!(s.find('é'), Some(14));
  assert_eq!(s.find("pard"), Some(17));
```

  More complex patterns using point-free style and closures:

```rust
  let s = "Löwe 老虎 Léopard";
  
  assert_eq!(s.find(char::is_whitespace), Some(5));
  assert_eq!(s.find(char::is_lowercase), Some(1));
  assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
  assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
```

  Not finding the pattern:

```rust
  let s = "Löwe 老虎 Léopard";
  let x: &[_] = &['1', '2'];
  
  assert_eq!(s.find(x), None);
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

  

---

### std::trim

- Description

  文字列スライスから先頭と末尾のホワイトスペースを削除したものを返します。

  空白は、`Unicode Derived Core Property White_Space`の条件に従って定義されます。

---

### f64::hypot

- Description

  長さ`x`と`y`の辺が与えられた直角三角形の斜辺の長さを計算します。

- Example

```rust
  let x = 2.0_f64;
  let y = 3.0_f64;
  
  // sqrt(x^2 + y^2)
  let abs_difference = (x.hypot(y) - (x.powi(2) + y.powi(2)).sqrt()).abs();
  
  assert!(abs_difference < 1e-10);
```

  

---

### slice

---

#### slice::windows

- Description

  長さ`size`のすべての連続したウィンドウのイテレータを返します。ウィンドウは重なっています。スライスがsize`よりも短い場合、イテレータは値を返しません。

- Example

```rust
  let slice = ['r', 'u', 's', 't'];
  let mut iter = slice.windows(2);
  assert_eq!(iter.next().unwrap(), &['r', 'u']);
  assert_eq!(iter.next().unwrap(), &['u', 's']);
  assert_eq!(iter.next().unwrap(), &['s', 't']);
  assert!(iter.next().is_none());
```

  

---

#### slice::contains

- Descriptiom

  スライスに指定した値の要素が含まれている場合はtrueを返します。

---

#### slice::starts_with

- Description

  needleがスライスの接頭辞である場合にtrueを返します。

---

#### slice::ends_with

- Description

  needleがスライスの接尾辞である場合にtrueを返します。



---

#### slice::sort

- Description

  スライスをソートします。

  このソートは安定しており（すなわち、等しい要素を並べ替えない）、最悪の場合`O(n * log(n))`となります。

  不安定なソートは、一般的に安定したソートよりも高速で、補助的なメモリを割り当てないので、適用可能な場合は、不安定なソートが好ましいです。[`sort_unstable`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort_unstable)を参照してください。

- 現在の実装

  現在のアルゴリズムは，[`timsort`](https://en.wikipedia.org/wiki/Timsort)にヒントを得た適応的な反復型マージソートです．これは、スライスがほぼソートされている場合や、2つ以上のソートされたシーケンスを次々と連結して構成されている場合に、非常に高速になるように設計されています。

  また、`self`の半分の大きさの一時ストレージを割り当てますが、短いスライスの場合は、代わりに非割り当ての挿入ソートを使用します。

- Example

```rust
  let mut v = [-5, 4, 1, -3, 2];
  
  v.sort();
  assert!(v == [-5, -3, 1, 2, 4]);
```

  

---

#### slice::sort_by

- Description

  スライスをコンパレータ関数でソートします。

  このソートは安定しており（つまり、等しい要素を並べ替えない）、最悪の場合は`O(n * log(n))`です。

  コンパレータ関数は、スライス内の要素の合計順序を定義しなければなりません。順序が合計でない場合、要素の順序は指定されません。順番は、（すべての`a`、`b`、`c`について）である場合、合計の順番です。

  - total and antisymmetric: `a < b`, `a == b`, `a > b` のうちの1つが正確に真で
  - transitive:`a < b` と`b < c` が含まれるとき `a < c`. 同じことが`==`と`>`にも当てはまります。

  例えば、`f64`は`NaN != NaN`なので`Ord`を実装していませんが、スライスに`NaN`が含まれていないことがわかっている場合は、`partial_cmp`をソート関数として使用することができます。

```rust
  let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
  floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
  assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
```

  不安定なソートは、一般的に安定したソートよりも高速で、補助的なメモリを割り当てないため、適用可能な場合は、不安定なソートが推奨されます。[`sort_unstable_by`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort_unstable_by) を参照してください。
  
- 現在の実装

  現在のアルゴリズムは，[`timsort`](https://en.wikipedia.org/wiki/Timsort)にヒントを得た適応的な反復型マージソートです．これは、スライスがほぼソートされている場合や、2つ以上のソートされたシーケンスを次々と連結して構成されている場合に、非常に高速になるように設計されています。

  また、`self`の半分の大きさの一時ストレージを割り当てますが、短いスライスの場合は、代わりに非割り当ての挿入ソートを使用します。

- Example

```rust
  let mut v = [5, 4, 1, 3, 2];
  v.sort_by(|a, b| a.cmp(b));
  assert!(v == [1, 2, 3, 4, 5]);
  
  // reverse sorting
  v.sort_by(|a, b| b.cmp(a));
  assert!(v == [5, 4, 3, 2, 1]);
```

---

#### slice::sort_by_key

- Description

  スライスをキー抽出関数でソートします。

  このソートは安定しており（すなわち、等しい要素を並べ替えない）、キー関数が`O(m)`である場合、最悪のケースでは`O(m * n * log(n))`となります。

  高価なキー関数（単純なプロパティアクセスや基本操作ではない関数など）では、要素のキーを再計算しないため、`sort_by_cached_key`の方が大幅に高速になる可能性があります。

  不安定なソートは、一般的に安定したソートよりも高速で、補助メモリを割り当てないため、適用可能な場合は、不安定なソートが好ましい。`sort_unstable_by_key`を参照してください。

- Current implementation

  現在のアルゴリズムは，[`timsort`](https://en.wikipedia.org/wiki/Timsort)にヒントを得た適応的な反復型マージソートです．これは、スライスがほぼソートされている場合や、2つ以上のソートされたシーケンスを次々と連結して構成されている場合に、非常に高速になるように設計されています。

  また、`self`の半分のサイズの一時ストレージを割り当てますが、短いスライスの場合は、代わりに非割り当ての挿入ソートを使用します。

- Example

```rust
  let mut v = [-5i32, 4, 1, -3, 2];
  
  v.sort_by_key(|k| k.abs());
  assert!(v == [1, 2, -3, 4, -5]);
```



---

#### slice::iter_mut

- Description

  各値を変更できるようにするイテレータを返します。



---

#### slice::iter

- Description

  スライスのイテレータを返します。

- Example

```rust
  let x = &[1, 2, 4];
  let mut iterator = x.iter();
  
  assert_eq!(iterator.next(), Some(&1));
  assert_eq!(iterator.next(), Some(&2));
  assert_eq!(iterator.next(), Some(&4));
  assert_eq!(iterator.next(), None);
```




---

#### slice::get

- Descroption

  インデックスのタイプに応じて、要素またはサブスライスへの参照を返します。

  - 位置が指定されている場合は、その位置にある要素への参照を返します。
  - 範囲が指定されている場合は、その範囲に対応するサブスライスを返し、範囲外の場合は`None`を返します。

- Example

```rust
  let v = [10, 40, 30];
  assert_eq!(Some(&40), v.get(1));
  assert_eq!(Some(&[10, 40][..]), v.get(0..2));
  assert_eq!(None, v.get(3));
  assert_eq!(None, v.get(0..4));
```



---

#### slice::join

- Description

  `T`のスライスを単一の値`Self::Output`にフラット化し、それぞれの間に指定されたセパレータを配置します。

- Example

```rust
  assert_eq!(["hello", "world"].join(" "), "hello world");
  assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
  assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
```




---

#### slice::into_vec

- Description

  クローンやアロケーションを使わずに`self`を`vector`に変換します。

  変換されたベクトルは、`Vec<T>`の`into_boxed_slice`メソッドによって、再び`Box<T>`に変換することができます。

- Example

```rust
  let s: Box<[i32]> = Box::new([10, 40, 30]);
  let x = s.into_vec();
  // `s` cannot be used anymore because it has been converted into `x`.
  
  assert_eq!(x, vec![10, 40, 30]);
```



---

### char

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

### std::any

- Description

  このモジュールは、`Any trait`を実装しています。`Any trait`は、ランタイムリフレクションによって任意の「静的な型」の動的な型付けを可能にします。

  `Any`自身は`TypeId`を取得するために使用できますが、`trait`オブジェクトとして使用するとさらに多くの機能があります。`&dyn Any`(借用した trait オブジェクト) としては、`is`および `downcast_ref `メソッドがあり、含まれる値が所定の型であるかどうかをテストしたり、内部の値を型として参照することができます。また、`&mut dyn Any`として、内部値への`mutable`な参照を取得する`downcast_mut`メソッドもあります。`Box<dyn Any>`には、`Box<T>`への変換を試みる`downcast`メソッドが追加されています。詳細は`Box`のドキュメントを参照してください。

  なお、`&dyn Any`は、値が指定された具象型であるかどうかのテストに限定されており、型が`trait`を実装しているかどうかのテストには使用できません。

- Example

  関数に渡された値をログアウトさせたい場合を考えてみましょう。対象となる値がDebugを実装していることはわかっていますが、その具体的な型はわかりません。特定の型に対して特別な扱いをしたいと考えています。この例では、`String`値の長さを値の前に表示しています。コンパイル時には値の具体的な型がわからないので、代わりにランタイム・リフレクションを使用する必要があります。

~~~rust
  use std::fmt::Debug;
  use std::any::Any;
  
  // Logger function for any type that implements Debug.
  fn log<T: Any + Debug>(value: &T) {
      let value_any = value as &dyn Any;
  
      // Try to convert our value to a `String`. If successful, we want to
      // output the String`'s length as well as its value. If not, it's a
      // different type: just print it out unadorned.
      match value_any.downcast_ref::<String>() {
          Some(as_string) => {
              println!("String ({}): {}", as_string.len(), as_string);
          }
          None => {
              println!("{:?}", value);
          }
      }
  }
  
  // This function wants to log its parameter out prior to doing work with it.
  fn do_work<T: Any + Debug>(value: &T) {
      log(value);
      // ...do some other work
  }
  
  fn main() {
      let my_string = "Hello World".to_string();
      do_work(&my_string);
  
      let my_i8: i8 = 100;
      do_work(&my_i8);
  }
~~~

---

#### std::any::TypeId

- Description

  `TypeId`は、あるタイプのグローバルに一意な識別子を表します。

  各`TypeId`は不透明なオブジェクトで、中身を確認することはできませんが、複製、比較、印刷、表示などの基本的な操作は可能です。

  `TypeId`は現在、'static'に準拠した型でのみ利用可能ですが、この制限は将来的に削除される可能性があります。

  `TypeId`は`Hash`、`PartialOrd`、`Ord`を実装していますが、ハッシュや順序は`Rust`のリリースごとに異なることに注意してください。コードの中でこれらに依存することに注意してください。

- Implementation

~~~rust
  pub fn of<T>() -> TypeId
  where
      T: 'static + ?Sized, 
  
~~~

  このジェネリック関数がインスタンス化された型のTypeIdを返します。

  - Example

    ---

~~~rust
    use std::any::{Any, TypeId};
    
    fn is_string<T: ?Sized + Any>(_s: &T) -> bool {
        TypeId::of::<String>() == TypeId::of::<T>()
    }
    
    assert_eq!(is_string(&0), false);
    assert_eq!(is_string(&"cookie monster".to_string()), true);
~~~

---

### std::time


---

#### std::time::Duration

  - Description

    Durationは、通常システムのタイムアウトに使用される時間のスパンを表す。
    各Durationは、秒の整数とナノ秒で表される端数で構成される。基礎となるシステムがナノ秒レベルの精度をサポートしていない場合、システム タイムアウトをバインディングするAPIは通常、ナノ秒数を切り上げる。
    Durationは、Add、Sub、その他のopsトレイトなど、多くの一般的なトレイトを実装している。長さ0のDurationを返すことでDefaultを実装している。

---

#### std::time::Instant

- Description

  単調に減少するクロックの測定値です。不透明で、Durationでのみ使用できます。

  インスタントは、作成された時点で、それまでに計測されたどの瞬間よりも小さくならないことが常に保証されており、ベンチマークの計測や、操作にかかる時間の計測などの作業によく使われます。

  ただし、インスタンスは安定していることは保証されていません。言い換えれば、基礎となる時計の各刻みは同じ長さではないかもしれません（例えば、ある秒は他の秒よりも長いかもしれません）。インスタントは、前方にジャンプしたり、時間拡張（遅くなったり、速くなったり）することはあっても、後方に戻ることはありません。

  インスタントは、お互いに比較することしかできない不透明なタイプです。インスタントから「何秒」を得る方法はありません。代わりに、2つのインスタント間の継続時間を測定する（または2つのインスタントを比較する）ことができるだけです。

  `Instant`構造体のサイズは、ターゲットのオペレーティングシステムによって異なる場合があります。

- Example

~~~rust
  use std::time::{Duration, Instant};
  use std::thread::sleep;
  
  fn main() {
     let now = Instant::now();
  
     // we sleep for 2 seconds
     sleep(Duration::new(2, 0));
     // it prints '2'
     println!("{}", now.elapsed().as_secs());
  }
~~~

---

### std::hash

---

#### `std::hash::Hash`

- Description

  ハッシュ化可能な型です。

  Hashを実装した型は、Hasherのインスタンスでハッシュ化することができます。

- Implementing `Hash`

  すべてのフィールドがHashを実装していれば、`#[derive(Hash)]`で`Hash`を派生させることができます。結果として得られるハッシュは、各フィールドのハッシュを呼び出したときの値を組み合わせたものになります。

~~~rust
  #[derive(Hash)]
  struct Rustacean {
      name: String,
      country: String,
  }
~~~

  値がどのようにハッシュ化されるかをより細かく制御する必要がある場合は、もちろん自分でHash特性を実装することができます。

~~~rust
  use std::hash::{Hash, Hasher};
  
  struct Person {
      id: u32,
      name: String,
      phone: u64,
  }
  
  impl Hash for Person {
      fn hash<H: Hasher>(&self, state: &mut H) {
          self.id.hash(state);
          self.phone.hash(state);
      }
  }
~~~

- `Hash` and `Eq`

  HashとEqの両方を実装する際には、次のような性質が成り立つことが重要です。

~~~
  k1 == k2 -> hash(k1) == hash(k2)
~~~

  言い換えれば、2つのキーが等しい場合、それらのハッシュも等しくなければなりません。HashMapとHashSetはどちらもこの動作に依存しています。

  ありがたいことに、`#[derive(PartialEq, Eq, Hash)]`で`Eq`と`Hash`の両方を導出する際には、このプロパティの保持を心配する必要はありません。

- Required methods

~~~rust
  pub fn hash<H>(&self, state: &mut H)
  where
      H: Hasher,
~~~

  このタイプのスライスを与えられたHasherにフィードします。

  - Example

    ---

~~~rust
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    7920.hash(&mut hasher);
    println!("Hash is {:x}!", hasher.finish());
~~~

- Provided methods

~~~rust
  pub fn hash_slice<H>(data: &[Self], state: &mut H)
  where
      H: Hasher, 
  
~~~

  このタイプのスライスを与えられたHasherにフィードします。

  - Exmaple

    ---

~~~rust
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    let numbers = [6, 28, 496, 8128];
    Hash::hash_slice(&numbers, &mut hasher);
    println!("Hash is {:x}!", hasher.finish());
~~~



---

#### `std::hash::Hasher`

- Description

  任意のバイトストリームをハッシュ化するための`trait`です。

  `Hasher`のインスタンスは通常、データのハッシュ化の際に変更される状態を表します。

  `Hasher`は、生成されたハッシュを (`finish`を使って) 取得したり、整数やバイトのスライスを (`write`や`write_u8`などを使って) インスタンスに書き込むための、かなり基本的なインターフェースを提供します。ほとんどの場合、`Hasher`インスタンスは、`Hash trait`と一緒に使用されます。

- Example

~~~rust
  use std::collections::hash_map::DefaultHasher;
  use std::hash::Hasher;
  
  let mut hasher = DefaultHasher::new();
  
  hasher.write_u32(1989);
  hasher.write_u8(11);
  hasher.write_u8(9);
  hasher.write(b"Huh?");
  
  println!("Hash is {:x}!", hasher.finish());
~~~

---

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

  


---

### std::marker

---

#### std::marker::Sync

- Description

  スレッド間で参照を共有することが安全な型。

  この特性は、コンパイラが適切と判断した場合に自動的に実装されます。

  正確な定義は次のとおりです：型Tは、`&T`が`Send`である場合に限り、`Sync`です。言い換えれば、スレッド間で`&T`の参照を渡す際に未定義の動作（データレースを含む）が発生する可能性がない場合です。

  予想されるように、`u8`や`f64`のようなプリミティブな型はすべて`Sync`であり、タプルや構造体、`enum`のようなそれらを含む単純な集約型も同様です。基本的な Sync 型の例としては、`&T`のような「不変」型や、`Box<T>`, `Vec<T>`や他のほとんどのコレクション型のような単純な継承された可変型があります。(ジェネリックパラメータは、そのコンテナが`Sync`であるために`Sync`である必要があります。)

  この定義のやや意外な結果は、非同期のミューテーションを提供しているように見えても、`&mut T`は (`T`が`Sync`であれば) `Sync`であるということです。このトリックは、共有参照の背後にある変異可能な参照（つまり`& &mut T`）が、あたかも`& &T `のように読み取り専用になることです。したがって、データレースのリスクはありません。

  `Sync`でない型は、`Cell`や`RefCell`のように、スレッドセーフでない形で「内部の変異性」を持っているものです。これらの型は、不変の共有参照を通しても、その内容の変異を許します。例えば、`Cell<T>`の`set`メソッドは、`&self`を受け取るので、共有参照の`&Cell<T>`だけが必要です。このメソッドは同期を行わないので、`Cell`は`Sync`にはなりません。

  非`Sync`型のもうひとつの例は、参照カウントポインタ`Rc`です。任意の参照 `&Rc<T> `が与えられると、新しい`Rc<T>`をクローンし、非アトミックな方法で参照カウントを変更することができます。

  スレッドセーフな内部の変異性が必要な場合、Rustはアトミックなデータ型を提供し、`sync::Mutex`と`sync::RwLock`による明示的なロックも提供します。`sync::Mutex`と`sync::RwLock`による明示的なロックがあります。これらのデータ型は、いかなる変異もデータレースを引き起こさないことを保証します。同様に、`sync::Arc`は`Rc`のスレッドセーフな類似性を提供します。

  内部に変異性を持つ型は、共有参照を通して変異させることができる値の周りに `cell::UnsafeCell`ラッパーを使用しなければなりません。これを怠ると未定義の動作となります。例えば、`&T`から`&mut T`への変換は無効です。

  `Sync`の詳細については[Nomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html)を参照してください。

---

#### std::marker::Send

- Description

  スレッドの境界を越えて転送可能な型。

  この特性は、コンパイラが適切と判断した場合、自動的に実装されます。

  非`Send`型の例としては、参照カウントのポインタ`rc::Rc`があります。2つのスレッドが同じ参照カウント値を指す`Rc`を複製しようとすると、同時に参照カウントを更新しようとするかもしれませんが、`Rc`はアトミック演算を使用していないので、これは未定義の動作です。これは`Rc`がアトミックな操作をしないため、未定義の動作です。

  詳しくは[Nomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html)をご覧ください。

---

#### std::marker::PhantomData

- Description

  `T`型を所有しているかのように「振る舞う」ものをマークするために使用されるゼロサイズの型。
  型に`PhantomData<T>`フィールドを追加すると、実際には`T`型の値を格納していないにもかかわらず、あたかも`T`型の値を格納しているかのように振る舞うことをコンパイラに伝えます。この情報は、特定の安全プロパティを計算する際に使用されます。
  PhantomData<T>の使用方法については、[Nomicon](https://doc.rust-lang.org/nomicon/phantom-data.html)を参照してください。

- Example

  - Unused lifetime parameters

    おそらくPhantomDataの最も一般的な使用例は、未使用の寿命パラメータを持つ構造体で、通常は安全でないコードの一部として使用されます。例えば、ここには`*const T`型の2つのポインタを持つ`Slice`構造体があり、おそらくどこかの配列を指していると思われます。

~~~rust
    struct Slice<'a, T> {
        start: *const T,
        end: *const T,
    }
~~~

    この意図は、基礎となるデータはライフタイム`'a`に対してのみ有効なので、`Slice`は`'a`よりも長生きしてはいけないということです。しかし、この意図はコードでは表現されていません。ライフタイム`'a`の用途がないため、どのデータに適用されるのかが明確ではありません。これを修正するには、コンパイラに`Slice`構造体に参照`&'a T`が含まれているかのように動作するように指示します。

~~~rust
    use std::marker::PhantomData;
    
    struct Slice<'a, T: 'a> {
        start: *const T,
        end: *const T,
        phantom: PhantomData<&'a T>,
    }
~~~

    これにより、`T: 'a`というアノテーションが必要になり、T内の参照が有効期間`'a`にわたって有効であることを示します。
    `Slice`を初期化する際には、`Phantom`フィールドに`PhantomData`という値を指定するだけです。

~~~rust
    fn borrow_vec<T>(vec: &Vec<T>) -> Slice<'_, T> {
        let ptr = vec.as_ptr();
        Slice {
            start: ptr,
            end: unsafe { ptr.add(vec.len()) },
            phantom: PhantomData,
        }
    }
~~~

  - Unused type parameters

    構造体自体にはデータが存在しないにもかかわらず、未使用の型パラメータが存在し、構造体がどのようなデータに「関連付けられているか」を示すことがあります。ここでは、`FFI`でこのような問題が発生する例を示します。外部インターフェイスでは、異なるタイプの`Rust`値を参照するために`*mut()`型のハンドルを使用します。ハンドルをラップする`ExternalResource`構造体のファントム型パラメータを使用して`Rust`型を追跡します。

~~~rust
    use std::marker::PhantomData;
    use std::mem;
    
    struct ExternalResource<R> {
       resource_handle: *mut (),
       resource_type: PhantomData<R>,
    }
    
    impl<R: ResType> ExternalResource<R> {
        fn new() -> Self {
            let size_of_res = mem::size_of::<R>();
            Self {
                resource_handle: foreign_lib::new(size_of_res),
                resource_type: PhantomData,
            }
        }
    
        fn do_stuff(&self, param: ParamType) {
            let foreign_params = convert_params(param);
            foreign_lib::do_stuff(self.resource_handle, foreign_params);
        }
    }
~~~

  - Ownership and the drop check

    `PhantomData<T>`型のフィールドを追加することは、あなたの型が`T`型のデータを所有していることを示します。これは、あなたの型がドロップされたときに、`T`型の1つ以上のインスタンスをドロップする可能性があることを意味しています。これは、`Rust`コンパイラのドロップチェック解析に関係します。
    構造体が実際に`T`型のデータを所有していない場合は、所有権を示さないように`PhantomData<&'a T>`(理想的には)または`PhantomData<*const T>`(ライフタイムが適用されない場合)のような参照型を使用した方が良いでしょう。


---

### std::thread

- The threading model

実行中のRustプログラムは、ネイティブOSのスレッドのコレクションで構成されており、それぞれが独自のスタックとローカルステートを持っている。

スレッドは名前を付けることができ、低レベルの同期のためのいくつかの組み込みサポートを提供している。
スレッド間の通信は、チャンネル、Rust のメッセージ通過型、他の形式のスレッド同期、共有メモリデータ構造を介して行うことができる。特に、スレッドセーフであることが保証されている型は、原子的に参照カウントされたコンテナ`Arc`を使用してスレッド間で簡単に共有することができる。

Rust で致命的なロジックエラーが発生すると、スレッドパニックが発生し、その間にスレッドはスタックを巻き戻し、デストラクタを実行し、所有するリソースを解放します。これは、スレッドがスタックを巻き戻し、デストラクタを実行し、所有するリソースを解放することを意味する。パニックが捕捉されなかった場合、スレッドは終了しますが、オプションで`join`を使用して別のスレッドからパニックを検出することができる。パニックが捕捉されずにメインスレッドがパニックに陥った場合、アプリケーションは0以外の終了コードで終了します。
      Rust プログラムのメインスレッドが終了すると、他のスレッドが実行中であってもプログラム全体がシャットダウンします。しかし、このモジュールは自動的に子スレッドの終了を待つための便利な機能を提供している。
      
- Spawning a thread

新しいスレッドは`thread::spawn`関数を使って生成することができる。
    
```rust
use std::thread;

let child = thread::spawn(move || {
// some work here
});
// some work here
let res = child.join();
```

`join`メソッドは、子スレッドが生成した最終的な値の`Ok`を含む`thread::Result`を返し、子スレッドがパニックに陥った場合は`panic!`コールに与えられた値のErrを返す。

新しいスレッドを生成するスレッドと、生成されるスレッドの間には、親子関係はないことに注意してください。特に、生成されたスレッドは、産み出されたスレッドがメインスレッドでない限り、生成されたスレッドよりも長生きすることもあれば、しないこともあります。

- Configuring threads

新しいスレッドは、生成される前に`Builder`タイプで設定することができ、現在はスレッドの名前とスタックサイズを設定することができます。

```rust
use std::thread;

thread::Builder::new().name("thread1".to_string()).spawn(move || {
    println!("Hello, world!");
});
```

- The Thread type
    

スレッドは、2 つの方法のいずれかで取得できる `Thread`型を介して表現される。
    

1. `thread::current`関数を使用して、現在のスレッドを要求する。

2. `thread::current`関数は、このモジュールの`API`によってスポーンされないスレッドに対しても利用可能。

- Thread-local storage

このモジュールは、`Rust`プログラム用のスレッドローカルストレージの実装も提供する。スレッドローカルストレージは、プログラム内の各スレッドが独自のコピーを持つグローバル変数にデータを格納する方法。スレッドはこのデータを共有しないので、アクセスを同期させる必要はない。スレッドローカルキーは、それが含む値を所有し、スレッドが終了したときにその値を破棄します。thread_local! マクロで作成され、`'static`(借用ポインタはない)な値を含むことができる。これは、指定されたクロージャへの値への共有参照を生成するアクセサ関数`with`を提供します。スレッドローカルキーは、値への共有アクセスのみを許可している。ほとんどの値は、`Cell`型や`RefCell`型を用いて何らかの形式の内部変異性を利用したいと考えるであろう。


- Naming threads

スレッドは、識別のために関連付けられた名前を持つことができる。デフォルトでは、スポーンされたスレッドには名前が付けられていない。スレッドの名前を指定するには、`Builder`でスレッドをビルドし、希望するスレッド名を`Builder::name`に渡します。スレッド名をスレッド内から取得するには Thread::name を使用します。スレッド名が使用される例をいくつか挙げます。

1. 指定されたスレッドでパニックが発生した場合、そのスレッド名がパニックメッセージに表示される。

2. スレッド名は、該当する場合には OS に提供される (例: unix ライクなプラットフォームでは `pthread_setname_np`)。

- Stack size

スポーンされるスレッドのデフォルトのスタックサイズは 2 MiB ですが、この特定のスタックサイズは将来的に変更される可能性がある。スポーンされるスレッドのスタックサイズを手動で指定するには、2つの方法がある。

1. Builder でスレッドをビルドし、希望するスタックサイズを`Builder::stack_size`に渡す。

2. 環境変数`RUST_MIN_STACK`を、希望するスタックサイズを表す整数 (バイト単位)に設定する。`Builder::stack_size`の設定はこれをオーバーライドすることに注意。

メインスレッドのスタックサイズはRustによって決定されないことに注意。
    

---

#### std::thread::spawn

- Description

  新しいスレッドを生成し、そのための`JoinHandle`を返します。

  `JoinHandle`は、ドロップされると暗黙のうちに子スレッドを切り離します。この場合、子スレッドは親スレッドよりも長生きする可能性があります（親スレッドがメインスレッドの場合は別ですが、メインスレッドが終了するとプロセス全体が終了します）。さらに、`JoinHandle`は、子スレッドに参加するために使用できる`join`メソッドを提供します。子スレッドがパニックに陥った場合、`join`は`panic！`に与えられた引数を含む`Err`を返します。

  スタックサイズやスレッドの名前を指定したい場合は、代わりにこのAPIを使用してください。

  spawnのシグネチャを見るとわかるように、spawnに与えられるクロージャとその戻り値の両方に2つの制約があります。

  - `'static`制約とは、クロージャとその戻り値は、プログラムの実行全体の寿命を持たなければならないということです。その理由は、スレッドが切り離され、作成されたライフタイムよりも長くなることがあるからです。実際、スレッドとその戻り値が呼び出し元よりも長くなる可能性がある場合、その後も有効であることを確認する必要があります。また、いつ戻り値が返されるかわからないので、できるだけ長く、つまりプログラムの終わりまで有効である必要があり、それが「静的寿命」の理由です。
  - `Send`制約は、クロージャが生成されたスレッドから新しいスレッドに値を渡す必要があるからです。クロージャの戻り値は、新しいスレッドからそれが結合されたスレッドに渡される必要があります。覚えておいていただきたいのですが、`Send`マーカートレイトはスレッドからスレッドに渡されても安全であることを表現しています。Sync は、スレッドからスレッドへ参照を渡すことが安全であることを表現しています。

- Panics

  このようなエラーから回復するには`Builder::spawn`を使用してください。

- Example

  Creating a thread.

```rust
  use std::thread;
  
  let handler = thread::spawn(|| {
      // thread code
  });
  
  handler.join().unwrap();
```

  モジュールのドキュメントにあるように、スレッドは通常[チャンネル](https://doc.rust-lang.org/stable/std/sync/mpsc/index.html)を使って通信するようになっており、以下のようになっています。

  この例では、値の所有権をスレッドに与えるために、`move`を使用する方法も示しています。

```rust
  use std::thread;
  use std::sync::mpsc::channel;
  
  let (tx, rx) = channel();
  
  let sender = thread::spawn(move || {
      tx.send("Hello, thread".to_owned())
          .expect("Unable to send on channel");
  });
  
  let receiver = thread::spawn(move || {
      let value = rx.recv().expect("Unable to receive from channel");
      println!("{}", value);
  });
  
  sender.join().expect("The sender thread has panicked");
  receiver.join().expect("The receiver thread has panicked");
```

  スレッドは[`JoinHandle`](https://doc.rust-lang.org/stable/std/thread/struct.JoinHandle.html)を通じて値を返すこともでき、これを使って非同期の計算を行うことができます（futuresの方がより適切かもしれません）。



---

#### [std](https://doc.rust-lang.org/stable/std/index.html)::[thread](https://doc.rust-lang.org/stable/std/thread/index.html)::JoinHandle

- Description

  スレッドに参加するために所有されているパーミッション(終了時にブロック)。

  `JoinHandle`は、関連するスレッドが削除されたときに関連するスレッドを切り離します。

  プラットフォームの制限のため、このハンドルを複製することはできません: スレッドに参加する能力は、一意に所有する権限です。

  この構造体は `thread::spawn`関数と`thread::Builder::spawn`メソッドによって作成されます。

---

#### std::thread::JoinHandle::join

  - Description

    関連するスレッドの終了を待つ。
    アトミックメモリの順序付けの観点からは、関連付けられたスレッドの完了は、この関数のリターンと同期する。
    言い換えれば、そのスレッドによって実行されたすべての操作は、joinが戻ってきた後に発生するすべての操作よりも先に順序付けられる。
    子スレッドがパニックに陥った場合、`panic！`に与えられたパラメータで`Err`が返される。
    この関数は、プラットフォームによってはスレッドが自分自身に参加しようとした場合にパニックを起こすかもしれないし、そうでなければスレッドの`join`でデッドロックを起こすかもしれない。

---

#### std::thread::sleep

  - Description

    現在のスレッドを、少なくとも指定した時間だけスリープ状態にする。
    スレッドは、スケジューリングの仕様やプラットフォーム依存の機能のために、指定された時間よりも長くスリープすることがある。スレッドのスリープ時間が短くなることはない。
    この関数はブロッキングであり、非同期関数では使用すべきではない。

---

#### std::thread::LocalKey

- Description

  そのコンテンツを所有するスレッドローカルストレージキー。

  このキーは、ターゲットプラットフォームで利用可能な最速の実装を使用します。このキーは[`thread_local!`](https://doc.rust-lang.org/std/macro.thread_local.html)マクロでインスタンス化され、主なメソッドは [with](https://doc.rust-lang.org/std/thread/struct.LocalKey.html#method.with)メソッドです。

  `with`メソッドは、含まれる値への参照を生成します。この参照は、スレッド間で送信したり、与えられたクロージャから抜け出すことはできません。

- Initialization and Destruction

  初期化は、スレッド内で`with`を最初に呼び出したときに動的に実行され、`Drop`を実装した値は、スレッドが終了するときに破棄されます。いくつかの注意点がありますので、以下に説明します。

  `LocalKey`のイニシャライザは、自分自身に再帰的に依存することはできません。この方法で`LocalKey`を使用すると、イニシャライザは`with`の最初の呼び出しで無限に再帰することになります。

- Example

```rust
  use std::cell::RefCell;
  use std::thread;
  
  thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
  
  FOO.with(|f| {
      assert_eq!(*f.borrow(), 1);
      *f.borrow_mut() = 2;
  });
  
  // each thread starts out with the initial value of 1
  let t = thread::spawn(move|| {
      FOO.with(|f| {
          assert_eq!(*f.borrow(), 1);
          *f.borrow_mut() = 3;
      });
  });
  
  // wait for the thread to complete and bail out on panic
  t.join().unwrap();
  
  // we retain our original value of 2 despite the child thread
  FOO.with(|f| {
      assert_eq!(*f.borrow(), 2);
  });
```

- Platform-specific behavior

  スレッドローカルストレージに格納されている型のデストラクタが実行されるように「最善の努力」が払われていますが、すべてのプラットフォームでスレッドローカルストレージのすべての型に対してデストラクタが実行されることが保証されているわけではないことに注意してください。例えば、デストラクタが実行されない既知の注意点がいくつかあります。

  1. Unixシステムで`pthread`ベースの TLS が使用されている場合、メイン・スレッドが終了する際に TLS の値に対してデストラクタが実行されません。メイン・スレッドが終了した直後に、アプリケーションも終了することに注意してください。
  2. すべてのプラットフォームで、TLSが破壊時に他のTLSスロットを再初期化することが可能です。一部のプラットフォームでは、破壊されたスロットの再初期化を防止することで、この現象が無限に起こらないようにしていますが、すべてのプラットフォームがこのガードを持っているわけではありません。ガードをしないプラットフォームでは、通常、合成制限を設けており、それを超えるとデストラクタは実行されません。



---

#### std::thread::LocalKey::with

- Description

  TLS キーの値への参照を取得します。

  このスレッドがまだこのキーを参照していない場合は、遅延的に値を初期化します。

- panics

  この関数は、キーに現在デストラクタが実行されている場合には`panic!()`し、以前にこのスレッドでデストラクタが実行されていた場合には`panic`することがあります。



---

### std::sync

---

#### std::sync::mpsc

  - Description

    このモジュールは、3 つのタイプの中で具体的に定義されたチャネルを介したメッセージベースの通信を提供します。

    - 送信者

    - シンクロ送信者

    - 受信機

    `Sender`または`SyncSender`は、`Receiver`にデータを送信するために使用される。どちらの送信者もクローン可能（Multi-producer）で、多くのスレッドが同時に 1 つのレシーバに送信することができる（single-consumer）。

    これらのチャンネルには 2 つの種類があります。

    - 非同期で無限にバッファリングされたチャンネルです。チャンネル関数は (Sender, Receiver) タプルを返します。チャネルは、概念的には無限のバッファを持つ。

    - 同期的な、制限されたチャンネル。`sync_channel`関数は (SyncSender, Receiver) タプルを返す。バッファスペースが空くまでブロックすることで、すべての送信は同期的に行われる。`0`の束縛が許されているため、チャネルは "rendezvous" channel となり、各送信者がメッセージを受信者にアトム単位で渡すようになることに注意。

    Disconnection
    チャンネルの送受信操作はすべて、操作が成功したかどうかを示す結果を返します。操作が成功しなかった場合は、通常はチャンネルの残りの半分が対応するスレッドに落とされて「ハングアップ」したことを示している。
    チャネルの半分が割り当てられてしまうと、ほとんどの操作は進行を続けることができなくなるため、`Err`が返されます。多くのアプリケーションでは、このモジュールから返された結果をアンラップし続け、あるスレッドが予期せず死んでしまった場合には、スレッド間で失敗が伝播してしまう。

---

#### std::sync::mpsc::channel

  - Description

    新しい非同期チャンネルを作成し、送信者と受信者の半分を返す。`Sender`で送信されたすべてのデータは、送信された順に `Receiver`で利用可能になり、`send`が呼び出し元のスレッドをブロックすることはない(このチャンネルには "無限のバッファ" があり、バッファの限界に達するとブロックされる `sync_channel`とは異なる)。
    `Sender` をクローンして同じチャンネルに複数回送信することができますが、サポートされているのは 1 つの`Receiver`のみ。

    `Sender`で送信しようとしている間に`Receiver`が切断された場合、送信メソッドは `SendError`を返す。同様に、`Sender`が`recv`しようとしているときに切断された場合、`recv`メソッドは`RecvError`を返す。

---

#### std::sync::mpsc::Sender::send

  - Description

    このチャネルに値を送信しようとし、送信できなかった場合は値を返す。
    送信が成功した場合は、チャンネルの相手側ハングアップしていないと判断された場合。送信に失敗した場合は、対応するチャンネルが既に割り当て解除されている場合。`Err`の戻り値はデータを受信しないことを意味し、`Ok`の戻り値はデータを受信することを意味しないことに注意すること。この関数が`Ok`を返した直後に、対応するチャンネルがハングアップする可能性がある。
    このメソッドは、現在のスレッドをブロックすることはない。

---

#### std::sync::mpsc::Receive::recv

  - Description

    この受信機で値の待ち受けを試み、対応するチャンネルがハングアップした場合はエラーを返す。
    この関数は、利用可能なデータがなく、より多くのデータを送信できる可能性がある場合、常に現在のスレッドをブロックする。対応する`Sender`(または`SyncSender`) にメッセージが送信されると、このレシーバはウェイクアップしてそのメッセージを返す。
    対応する`Sender`が切断された場合や、このコールがブロックされている間に切断された場合は、このコールはウェイクアップして`Err`を返し、このチャンネルではこれ以上メッセージを受信できないことを示す。ただし、チャネルはバッファリングされているので、切断前に送信されたメッセージは正しく受信される。

---

#### std::sync::Mutex

  - Description

  共有データの保護に有用な相互排除プリミティブ
  このmutexは、ロックが利用可能になるのを待つスレッドをブロックする。`mutex`は静的に初期化したり、新しいコンストラクタを使って作成することもできます。各`mutex`には保護するデータを表す`type`パラメータがあります。データは`lock`と`try_lock`から返される RAII ガードを介してのみアクセスでき、`mutex`がロックされているときにのみデータにアクセスできることを保証する。

  - Poisoning
    このモジュールのmutexは「Poisoning」と呼ばれる戦略を実装しており、mutexを保持している間にスレッドがパニックになると、いつでもmutexがポイズニングされているとみなされます。一度mutexがポイズニングされると、他のすべてのスレッドはデータが汚染されている可能性が高いので、デフォルトではデータにアクセスできなくなります(何らかの不変量が保持されていない)。
    mutexの場合、これは`lock`メソッドと`try_lock`メソッドが、`mutex`がポイズンされたかどうかを示す `Result`を返すことを意味します。mutexのほとんどの使用法では、これらの結果を単に unwrap() して、無効な不変量が目撃されないようにスレッド間でパニックを伝播させる。
    しかし、ポイズンされたmutexは、基礎となるデータへのすべてのアクセスを妨げるものではない。`PoisonError`型には`into_inner`メソッドがあり、これはロックが成功したときに返されるはずのガードを返す。これにより、ロックがポイズンされているにもかかわらず、データへのアクセスが可能になる。



---

#### std::sync::Mutex::lock

  - Description

    mutexを取得し、それが可能になるまで現在のスレッドをブロックします。

    この関数は、mutexを取得できるようになるまでローカルスレッドをブロックします。解放時には、そのスレッドはロックが保持されている唯一のスレッドとなります。ロックのスコープ付きアンロックを可能にするために、`RAII`ガードが返されます。ガードがスコープ外になると、mutexはアンロックされる。
    既にロックを保持しているスレッドでmutexをロックする場合の正確な動作は未定義である。しかし、この関数は2回目の呼び出しでは戻りません(例えば、パニックやデッドロックになる可能性がある)。

    - Error
      このmutexを保持している間にこのmutexの他のユーザがパニックに陥った場合、この呼び出しはmutexを取得した後にエラーを返す。
    - Panic
      この関数は、現在のスレッドが既にロックを保持している場合に呼び出されるとパニックになる可能性がある。

---

#### std::sync::Arc

  - Description

    スレッドセーフな参照カウントポインタ。`Arc`は`Atomically Reference Counted`の略。

    `Arc<T>`型は、ヒープに割り当てられた`T`型の値の共有所有権を提供する。`Arc`上で`clone`を実行すると、参照カウントを増加させながら、ソース`Arc`と同じヒープ上の割り当てを指す新しい`Arc`インスタンスが生成される。与えられたアロケーションへの最後の`Arc`ポインタが破棄されると、そのアロケーションに格納されている値 (多くの場合、「内部値」と呼ばれます) も削除されます。

    Rust の共有参照はデフォルトで突然変更されることを禁止しており、`Arc`も例外ではありません。`Arc`を通してミューテーションを行う必要がある場合は、`Mutex`、`RwLock`、または`Atomic`型のいずれかを使用してください。

    `Rc<T>`とは異なり、`Arc<T>`は参照カウントにアトミック演算を使用します。これはスレッドセーフであることを意味します。欠点は、アトミック演算が通常のメモリアクセスに比べて高価なことです。スレッド間で参照カウントされた割り当てを共有しない場合は、より低いオーバーヘッドのために`Rc<T>`の使用を検討してください。スレッド間で`Rc<T> `を送ろうとすると、コンパイラがそれをキャッチするので、`Rc<T>`は安全なデフォルトです。しかし、ライブラリの利用者に柔軟性を持たせるために、ライブラリは`Arc<T>`を選択するかもしれません。

    `Arc<T>`は、`T`が`Send`と`Sync`を実装している限り、`Send`と`Sync`をする。スレッドセーフではない型の`T`を`Arc<T>`に入れてスレッドセーフにすることができないのはなぜか？最初は少し直観的ではないかもしれないが、結局のところ、`Arc<T>` のスレッドセーフは重要ではないのではないのか？結局のところ、`Arc<T>`のスレッド安全性は重要ではないのではないのか？重要なのは、`Arc<T>`は、同じデータの複数の所有権を持つことをスレッドセーフにする、そのデータにスレッドセーフを追加するわけではない。`Arc<RefCell<T>>`を考えてみる。`RefCell<T>`は`Sync`ではないので、もし`Arc<T>`が常に`Send`であれば、`Arc<RefCell<T>`も同様に`Send`になります。しかし、そうすると問題が発生する。`RefCell<T>`はスレッドセーフではない。

    `RefCell<T>`はスレッドセーフではないので、非アトミック演算を使って借用回数を追跡する。

    `downgrade`メソッドを使用して、所有権のない`Weak`ポインタを作成することができる。`Weak`ポインタを`Arc`にアップグレードすることができますが、アロケーションに格納されている値が既にドロップされている場合は`None`を返す。言い換えれば、`Weak` ポインタはアロケーション内の値を保持しませんが、アロケーション (値の裏付けとなるストア) を保持する。

    `Arc`ポインタ間のサイクルは決して解放されない。このため、`Weak`はサイクルを壊すために使用されます。例えば、ツリーは親ノードから子ノードへの強いアークポインタを持ち、子ノードから親ノードへの弱いポインタを持つことができる。

    - Cloning
      既存の参照カウントされたポインタから新しい参照を作成するには、`Arc<T>`と`Weak<T>`に実装された`Clone`トレイトを使用します。

~~~rust
    use std::sync::Arc;
    let foo = Arc::new(vec![1.0, 2.0, 3.0]);
    // The two syntaxes below are equivalent.
    let a = foo.clone();
    let b = Arc::clone(&foo);
    // a, b, and foo are all Arcs that point to the same memory location
~~~

    - Deref behavior
      `Arc<T>`は自動的に (`Deref`トレイトを介して) `T`に派生するので、`Arc<T>`型の値に対して`T`のメソッドを呼び出すことができる。`T`のメソッドとの名前の衝突を避けるため、`Arc<T>`のメソッドは関連する関数であり、完全修飾構文を用いて呼び出される。

~~~rust
    use std::sync::Arc;
    
    let my_arc = Arc::new(());
    Arc::downgrade(&my_arc);
~~~

    `Clone` のようなトレイトの`Arc<T>`の実装も、完全修飾構文を使って呼ばれることがある。

~~~rust
    use std::sync::Arc;
    
    let arc = Arc::new(());
    // Method-call syntax
    let arc2 = arc.clone();
    // Fully qualified syntax
    let arc3 = Arc::clone(&arc);
~~~

    `Weak<T>`は、内部の値が既にドロップされている可能性があるため、`T`への自動参照は行わない。

---

#### std::sync::Arc::new

- Description

  新しい`Arc<T>`を構築します。

- Example

```rust
  use std::sync::Arc;
  
  let five = Arc::new(5);
```



---

#### std::sync::Arc::clone

- Description

  `Arc`ポインタのクローンを作成します。

  これにより、同じアロケーションへの別のポインタが作成され、強い参照カウントが増加します。

- Example

```rust
  use std::sync::Arc;
  
  let five = Arc::new(5);
  
  let _ = Arc::clone(&five);
```



---

#### std::sync::RwLock

- Description

  リーダライタロック

  このタイプのロックでは、任意の時点で複数のリーダーまたは最大1つのライターを許可します。このロックの書き込み部分は、通常、基礎となるデータの変更を可能にし（排他的アクセス）、このロックの読み取り部分は、通常、読み取り専用のアクセスを可能にします（共有アクセス）。

  一方、`Mutex`は、ロックを取得するリーダーとライターを区別しないため、ロックが利用可能になるのを待つスレッドがブロックされます。`RwLock`は、ライターがロックを保持していない限り、何人のリーダーでもロックを取得することができます。

  ロックの優先ポリシーは、基礎となるオペレーティングシステムの実装に依存しており、このタイプは特定のポリシーが使用されることを保証するものではありません。特に、書き込み時にロックの取得を待っているライターは、同時に行われる読み取りの呼び出しをブロックする場合もありますし、しない場合もあります。

  型パラメータTは、このロックが保護するデータを表します。`T`は、スレッド間で共有するための`Send`と、リーダーによる同時アクセスを可能にするための`Sync`を満たすことが要求されます。ロックメソッドから返される RAII ガードは、ロックの内容へのアクセスを許可するために `Deref `(および`write`メソッドの `DerefMut`) を実装しています。

- Poisoning

  `RwLock`は、`Mutex`と同様に、パニックが発生するとポイズンになります。ただし、`RwLock`がポイズンされるのは、それが排他的にロックされている間（書き込みモード）にパニックが発生した場合のみであることに注意してください。パニックがリーダーで発生した場合は、ロックはポイズニングされません。

- Example

```rust
  use std::sync::RwLock;
  
  let lock = RwLock::new(5);
  
  // many reader locks can be held at once
  {
      let r1 = lock.read().unwrap();
      let r2 = lock.read().unwrap();
      assert_eq!(*r1, 5);
      assert_eq!(*r2, 5);
  } // read locks are dropped at this point
  
  // only one write lock may be held, however
  {
      let mut w = lock.write().unwrap();
      *w += 1;
      assert_eq!(*w, 6);
  } // write lock is dropped here
```




---

#### std::sync::RwLock::new

- Description

  ロックが解除された`RwLock<T>`の新しいインスタンスを作成します。

- Example

```rust
  use std::sync::RwLock;
  
  let lock = RwLock::new(5);
```



---

#### std::sync::RwLock::read

- Description

  `rwlock`を共有の読み取りアクセスでロックし、それを取得できるまで現在のスレッドをブロックします。

  呼び出したスレッドは、ロックを保持するライターが無くなるまでブロックされます。このメソッドが戻るときには、現在ロックの内側に他のリーダーがいるかもしれません。このメソッドは、競合するリーダとライタのどちらが先にロックを取得するかという順序に関して、いかなる保証も行いません。

  このスレッドの共有アクセスが削除されると、それを解放する`RAII`ガードを返します。

- Errors

  この関数は、`RwLock`がポイズンされた場合、エラーを返します。`RwLock`は、排他的ロックを保持している間にライターがパニックを起こすと、ポイズンされます。ロックを取得した直後に障害が発生します。

- Panics

  この関数は、ロックがすでに現在のスレッドによって保持されている場合、呼び出されるとパニックになることがあります。

- Example

```rust
  use std::sync::{Arc, RwLock};
  use std::thread;
  
  let lock = Arc::new(RwLock::new(1));
  let c_lock = Arc::clone(&lock);
  
  let n = lock.read().unwrap();
  assert_eq!(*n, 1);
  
  thread::spawn(move || {
      let r = c_lock.read();
      assert!(r.is_ok());
  }).join().unwrap();
```



---

#### std::sync::RwLock::write

- Description

  `rwlock`を排他的な書き込みアクセスでロックし、それが取得できるまで現在のスレッドをブロックします。

  この関数は、他のライターまたは他のリーダーが現在ロックにアクセスしている間は戻りません。

  ドロップされたときにこの`rwlock`の書き込みアクセスをドロップする`RAII`ガードを返します。

- Errors

  この関数は、`RwLock`がポイズンされた場合、エラーを返します。`RwLock`は、ライターが排他的ロックを保持している間にパニックになると、ポイズンされます。ロックを取得するとエラーが返されます。

- Panics

  この関数は、ロックがすでに現在のスレッドによって保持されている場合、呼び出されるとパニックになることがあります。

- Example

```rust
  use std::sync::RwLock;
  
  let lock = RwLock::new(1);
  
  let mut n = lock.write().unwrap();
  *n = 2;
  
  assert!(lock.try_read().is_err());
```



---

#### std::sync::atomic

- Description

  Atomic型

  アトミック型はスレッド間のプリミティブな共有メモリ通信を提供し、他の並行型の構成要素となります。
  このモジュールは、`AtomicBool`、`AtomicIsize`、`AtomicUsize`、`AtomicI8`、`AtomicU16`などを含む、選択された数のAtomic型のアトミックバージョンを定義します。Atomic型は、正しく使用されるとスレッド間の更新を同期させる操作を提供します。
  各メソッドは、その操作のためのメモリバリアの強さを表す順序を取ります。これらの順序付けは、C++20 のアトミック順序付けと同じです。詳細については、[nomicon](https://doc.rust-lang.org/stable/nomicon/atomics.html)を参照してください。
  Atomic変数はスレッド間で共有しても安全ですが（`Sync`を実装しています）、それ自体は共有のメカニズムを提供しておらず、Rustのスレッドモデルに従っています。アトミック変数を共有する最も一般的な方法は、`Arc`(原子的に参照カウントされた共有ポインタ) に格納することです。
  Atomic型は静的変数に格納され、`AtomicBool::new`のような定数初期化子を使って初期化されます。Atomic静的変数は、遅延グローバル初期化によく使われます。

---

#### std::sync::Condvar

- Description

  コンディション変数

  コンディション変数とは、あるイベントの発生を待つ間、CPU時間を消費しないようにスレッドをブロックする機能です。コンディション変数は通常、ブーリアン値の述語（コンディション）とミューテックスに関連付けられています。述語は、スレッドをブロックしなければならないと判断する前に、常にミューテックスの内部で検証されます。

  このモジュールの関数は、現在の実行スレッドをブロックします。同じ条件変数に複数のミューテックスを使おうとすると、ランタイム・パニックを起こす可能性があることに注意してください。

- Example

```rust
  use std::sync::{Arc, Mutex, Condvar};
  use std::thread;
  
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair2 = Arc::clone(&pair);
  
  // Inside of our lock, spawn a new thread, and then wait for it to start.
  thread::spawn(move|| {
      let (lock, cvar) = &*pair2;
      let mut started = lock.lock().unwrap();
      *started = true;
      // We notify the condvar that the value has changed.
      cvar.notify_one();
  });
  
  // Wait for the thread to start up.
  let (lock, cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  while !*started {
      started = cvar.wait(started).unwrap();
  }
```



---

#### std::sync::Condvar::wait

- Description

  この条件変数が通知を受け取るまで、現在のスレッドをブロックします。

  この関数は、指定された（guardで表される）ミューテックスをアトミックにアンロックし、現在のスレッドをブロックします。これは、ミューテックスのロックが解除された後に論理的に発生する`notify_one`や`notify_all`の呼び出しが、このスレッドを起こす候補になることを意味します。この関数呼び出しが戻るとき、指定されたロックは再取得されています。

  この関数は、偽のウェイクアップの影響を受けやすいことに注意してください。条件変数には通常、ブーリアン値の述語が関連付けられており、偽のウェイクアップから守るために、この関数が戻るたびに述語をチェックしなければなりません。

- Example

```rust
  use std::sync::{Arc, Mutex, Condvar};
  use std::thread;
  
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair2 = Arc::clone(&pair);
  
  thread::spawn(move|| {
      let (lock, cvar) = &*pair2;
      let mut started = lock.lock().unwrap();
      *started = true;
      // We notify the condvar that the value has changed.
      cvar.notify_one();
  });
  
  // Wait for the thread to start up.
  let (lock, cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  // As long as the value inside the `Mutex<bool>` is `false`, we wait.
  while !*started {
      started = cvar.wait(started).unwrap();
  }
```



---

#### std::sync::Condvar::notify_all

- Description

  この条件変数でブロックされているすべてのスレッドを起動させます。

  このメソッドは、条件変数の現在の待機者がすべて起こされることを保証します。`notify_all()`の呼び出しは、いかなる方法でもバッファリングされません。

  1つのスレッドだけを目覚めさせるには、`notify_one`を参照してください。

- Example

```rust
  use std::sync::{Arc, Mutex, Condvar};
  use std::thread;
  
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair2 = Arc::clone(&pair);
  
  thread::spawn(move|| {
      let (lock, cvar) = &*pair2;
      let mut started = lock.lock().unwrap();
      *started = true;
      // We notify the condvar that the value has changed.
      cvar.notify_all();
  });
  
  // Wait for the thread to start up.
  let (lock, cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  // As long as the value inside the `Mutex<bool>` is `false`, we wait.
  while !*started {
      started = cvar.wait(started).unwrap();
  }
```

---

#### std::sync::Barrier

- Description

  バリアは、複数のスレッドがある計算の開始を同期させるためのものです。

- Example

```rust
  use std::sync::{Arc, Barrier};
  use std::thread;
  
  let mut handles = Vec::with_capacity(10);
  let barrier = Arc::new(Barrier::new(10));
  for _ in 0..10 {
      let c = Arc::clone(&barrier);
      // The same messages will be printed together.
      // You will NOT see any interleaving.
      handles.push(thread::spawn(move|| {
          println!("before wait");
          c.wait();
          println!("after wait");
      }));
  }
  // Wait for other threads to finish.
  for handle in handles {
      handle.join().unwrap();
  }
```

---

#### std::sync::Barrier::wait

- Description

  すべてのスレッドがここでランデブーするまで、現在のスレッドをブロックします。

  バリアは、すべてのスレッドが一度ランデブーした後も再利用可能で、継続的に使用することができます。

  単一の（任意の）スレッドは、この関数から戻るときに`BarrierWaitResult::is_leader()`から`true`を返す `BarrierWaitResult`を受け取り、他のすべてのスレッドは`BarrierWaitResult::is_leader()`から`false`を返す結果を受け取ることになります。

- Example

```rust
  use std::sync::{Arc, Barrier};
  use std::thread;
  
  let mut handles = Vec::with_capacity(10);
  let barrier = Arc::new(Barrier::new(10));
  for _ in 0..10 {
      let c = Arc::clone(&barrier);
      // The same messages will be printed together.
      // You will NOT see any interleaving.
      handles.push(thread::spawn(move|| {
          println!("before wait");
          c.wait();
          println!("after wait");
      }));
  }
  // Wait for other threads to finish.
  for handle in handles {
      handle.join().unwrap();
  }
```




---

#### std::sync::atomic::AtomicUsize

- Description

  スレッド間で安全に共有することができる整数型。

  この型は、基礎となる整数型である`usize`と同じメモリ内表現を持ちます。アトミック型と非アトミック型の違いや、この型の移植性については、モジュールレベルのドキュメントを参照してください。

  注意：この型は、`usize`のアトミックなロードとストアをサポートするプラットフォームでのみ使用できます。

- Implementations

  - fetch_add

    - Description

      現在の値に加算し、前の値を返します。

      この操作は、オーバーフロー時に折り返します。

      `fetch_add`は、この操作のメモリ順序を記述する`Ordering`引数を取ります。すべての順序モードが可能です。`Acquire`を使用すると、この操作のストア部分がRelaxedになり、`Release`を使用するとロード部分が`Relaxed`になることに注意してください。

      注：このメソッドは、`usize`のアトミック操作をサポートするプラットフォームでのみ使用できます。

    - Example

    ```rust
      use std::sync::atomic::{AtomicUsize, Ordering};
      
      let foo = AtomicUsize::new(0);
      assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
      assert_eq!(foo.load(Ordering::SeqCst), 10);
    ```

  - fetch_sub

    - Description

      現在の値から減算し、前の値を返します。

      この操作は、オーバーフロー時に折り返します。

      `fetch_sub`は、この操作のメモリ順序を記述する`Ordering`引数を取ります。すべての順序モードが可能です。`Acquire`を使用すると、この操作のストア部分が`Relaxed`になり、`Release`を使用するとロード部分が`Relaxed`になることに注意してください。

      注：このメソッドは、`usize`のアトミック操作をサポートするプラットフォームでのみ使用できます。

    - Example

    ```rust
      use std::sync::atomic::{AtomicUsize, Ordering};
      
      let foo = AtomicUsize::new(20);
      assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);
      assert_eq!(foo.load(Ordering::SeqCst), 10);
    ```

  - load

    - Description

      Atomicな整数から値をロードします。

      `load`は、この操作のメモリ順序を記述する`Ordering`引数を取ります。可能な値は、`SeqCst`、`Acquire`、`Relaxed`です。

    - panics

      orderがが`Release`や`AcqRel`の場合はパニックになります。

    - Example

    ```rust
      use std::sync::atomic::{AtomicUsize, Ordering};
      
      let some_var = AtomicUsize::new(5);
      
      assert_eq!(some_var.load(Ordering::Relaxed), 5);
    ```

---

#### std::sync::atomic::Ordering

- Description

  アトミックメモリの順序付け

  メモリの順序付けは、原子演算がメモリを同期させる方法を指定します。最も弱い`Ordering::Relaxed`では、操作によって直接触れられたメモリだけが同期される。一方、`Ordering::SeqCst`のストア・ロードのペアは、他のメモリを同期させ、さらにすべてのスレッドでそのような操作の合計順序を保持します。

  Rust のメモリ順序は、C++20のものと同じです。

  詳細は[nomicon](https://doc.rust-lang.org/nomicon/atomics.html)を参照してください。

- Variants 

  - Relaxed
    順序の制約はなく、アトミックな操作のみ。

    C++20 の`memory_order_relaxed`に相当します。

  - Release

    ストアと組み合わせた場合、以前のすべての操作は、`Acquire`（またはそれ以上）の順序でこの値のロードの前に順序付けられます。特に、以前の書き込みはすべて、この値の`Acquire`（またはそれ以上）のロードを実行するすべてのスレッドから見えるようになります。

    ロードとストアを組み合わせた操作にこの順序を使用すると、`Relaxed load`操作になることに注意してください。

    この順序は、ストアを実行できる操作にのみ適用されます。

    C++20 では`memory_order_release`に対応しています。

  - Acquire

    ロードと組み合わせた場合、ロードされた値が`Release`（またはより強い）順序のストア操作によって書き込まれた場合、後続のすべての操作はそのストアの後に順序付けられるようになります。特に、後続のロードはすべて、ストアの前に書き込まれたデータを見ることになります。

    ロードとストアを組み合わせた操作にこの順序を使用すると、`Relaxed store`操作になることに注意してください。

    この順序は、ロードを行うことができる操作にのみ適用されます。

    C++20 の`memory_order_acquire`に対応しています。

  - AcqRel

    `Acquire`と`Release`の両方の効果を併せ持つ。ロードでは`Acquire`順序を使用し、ストアでは`Release`順序を使用します。

    `compare_and_swap`の場合は、操作がストアを実行せずに終わる可能性があるため、Acquire 順序だけになっていることに注意してください。しかし、`AcqRel`が`Relaxed`アクセスを実行することはありません。

    この順序は、ロードとストアの両方を組み合わせた操作にのみ適用されます。

    C++20 の`memory_order_acq_rel`に対応しています。

  - SeqCst

    `Acquire`/`Release`/`AcqRel`（それぞれロード、ストア、ストア付きロードの操作）に、すべてのスレッドがすべての連続した操作を同じ順序で見ることができるという保証を追加したもの。

    C++20 の`memory_order_seq_cst`に対応しています。



---

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




---

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



---

#### std::net::TcpListener

- Description

  TCP ソケットサーバで、接続をリッスンします。

  ソケットアドレスにバインドしてTcpListenerを作成した後、着信TCP接続をリッスンします。これらは accept を呼び出すか、incoming で返された Incoming イテレータを反復処理することで受け入れることができます。

  値がドロップされるとソケットは閉じられます。

  送信制御プロトコルはIETF RFC 793で規定されています。

  - Example

~~~rust
    use std::net::{TcpListener, TcpStream};
    
    fn handle_client(stream: TcpStream) {
        // ...
    }
    
    fn main() -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:80")?;
    
        // accept connections and process them serially
        for stream in listener.incoming() {
            handle_client(stream?);
        }
        Ok(())
    }
~~~

    

- Implementations

  - bind

    指定されたアドレスにバインドされる新しいTcpListenerを作成します。

    返されたリスナーは、接続を受け入れる準備ができています。

    ポート番号0でバインドすると、OSがこのリスナーにポートを割り当てるように要求します。割り当てられたポートは、`TcpListener::local_addr`メソッドで問い合わせることができます。

    アドレス型は`ToSocketAddrs`トレイトの任意の実装を指定することができます。具体的な例については、そのドキュメントを参照してください。

    `addr`が複数のアドレスを生成した場合、1つのアドレスが成功してリスナーを返すまで、それぞれのアドレスでバインドが試みられます。どのアドレスもリスナーの作成に成功しなかった場合、最後の試行 (最後のアドレス) から返されるエラーが返されます。

    

    - Example

      127.0.0.0.1:80 にバインドされた TCP リスナーを作成します。

~~~rust
      use std::net::TcpListener;
      
      let listener = TcpListener::bind("127.0.0.1:80").unwrap();
~~~

      127.0.0.0.1:80 にバインドされた TCP リスナーを作成します。失敗した場合は、127.0.0.0.1:443 にバインドされた TCP リスナーを作成します。

~~~rust
      use std::net::{SocketAddr, TcpListener};
      
      let addrs = [
          SocketAddr::from(([127, 0, 0, 1], 80)),
          SocketAddr::from(([127, 0, 0, 1], 443)),
      ];
      let listener = TcpListener::bind(&addrs[..]).unwrap();
~~~

  - incoming

    このリスナーで受信している接続のイテレータを返します。

    返されるイテレータは `None`を返すことはなく、相手の`SocketAddr`構造体も返しません。これを繰り返し処理することは、ループ内で`TcpListener::accept`を呼び出すことと同じです。

    - Example

~~~rust
      use std::net::TcpListener;
      
      let listener = TcpListener::bind("127.0.0.1:80").unwrap();
      
      for stream in listener.incoming() {
          match stream {
              Ok(stream) => {
                  println!("new client!");
              }
              Err(e) => { /* connection failed */ }
          }
      }
~~~



---

#### std::net::TcpStream

- Description

  ローカルとリモートのソケット間のTCPストリーム。

  リモートホストに接続するか、TcpListener上で接続を受け付けるかのいずれかでTcpStreamを作成した後、そこに読み書きすることでデータを送信することができます。

  値をドロップした時点で接続を終了します。また、接続の読み書き部分は、シャットダウンメソッドで個別にシャットダウンすることができます。

  伝送制御プロトコルはIETF RFC 793に規定されています。

  - Example

~~~rust
    use std::io::prelude::*;
    use std::net::TcpStream;
    
    fn main() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:34254")?;
    
        stream.write(&[1])?;
        stream.read(&mut [0; 128])?;
        Ok(())
    } // the stream is closed here
~~~

- 