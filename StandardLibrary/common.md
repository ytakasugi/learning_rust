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
