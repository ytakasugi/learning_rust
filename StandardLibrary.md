# 標準ライブラリ

本テキストは、Rustを学習していく中で使用する標準ライブラリについて、記載していく。

詳細は、[標準ライブラリの公式ドキュメント](https://doc.rust-lang.org/stable/std/)を参照のこと。

---

### Selfキーワード

  - Description
    trait または impl ブロック内の実装型、または型定義内の現在の型。

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
    1つのポインタはデータ(構造体のインスタンスなど)へのポインタ。もう 1 つのポインタは、メソッド呼び出し名と関数ポインタのマップ(仮想メソッドテーブルまたは vtable として知られている)へのポインタです。
    実行時に、`dyn Trait`上でメソッドを呼び出す必要がある場合、関数ポインタを取得するために`vtable`が参照され、その関数ポインタが呼び出される。

    - トレードオフ

      上記は間接的には、`dyn Trait`上で関数を呼び出す際の追加の実行コストである。動的ディスパッチによって呼び出されたメソッドは、一般的にコンパイラによってインライン化することができない。
      しかし、具体的な型ごとにメソッドが重複しないため、`dyn Trait`は`impl Trait / generic parameters` よりも小さなコードを生成する可能性がある。
      オブジェクトの安全性と traitオブジェクトについての詳細は[こちら](https://doc.rust-lang.org/stable/book/ch17-02-trait-objects.html)を参照。

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

- クローントレイトを実装する型

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

### core::result::Result::map

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

### std::result::Result::and_then

  - Description

    結果が`Ok`の場合は`op`を呼び出し、そうでない場合は`self`の`Err`値を返す

    


---

### [std::result::Result::is_err](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.is_err)

  - Description

    結果が`Err`なら`true`を返す。

---

### std::result::Result::map_err

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

### std::result::Result::unwrap_or_else

  - Description

    標準ライブラリで`Result<T, E>`に定義されている。

    unwrap_or_elseを使用することで、panic!ではない独自のエラーを返すことでができる。

    `Result<T,E>`の結果が`Ok`であれば、`Ok`が包んでいる値を返却する(`unwrap`に似ている)

    値が`Err`値なら、このメソッドはクロージャ内でコードを呼び、クロージャに定義した引数として`unwrap_or_else`に渡す匿名関数である。

---

### [std::error::Error](https://doc.rust-lang.org/std/error/trait.Error.html)

- Description

  エラーは、エラー値に対する基本的な期待を表すトレイトであり、`Result<T, E>`の`E`型の値である。エラーは、`Display`と`Debug`のトレイトを通して自分自身を記述しなければならず、原因の連鎖情報を提供することもあります。

  `Error::source()`は、エラーが「抽象化の境界」を越える場合に一般的に使用されます。あるモジュールが、下位モジュールのエラーに起因するエラーを報告しなければならない場合、`Error::source()`を介してそのエラーにアクセスすることを許可することができます。これにより、上位モジュールが独自のエラーを提供することが可能になり、同時にソースチェーンを介したデバッグのために実装の一部を明らかにすることができます。


---

### [std::option::Option::as_ref](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.as_ref) 

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

### [std::option::Option::map_or](https://doc.rust-lang.org/stable/core/option/enum.Option.html#method.map_or)

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

### std::option::Option::ok_or

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

### std::option::Option::ok_or_else

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

### std::option::Option::take

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

### std::convert::Into

- Description

  入力値を消費する値から値への変換のこと。\From\の反対です。

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

### std::convert::From

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

### std::io::BufReader<R>

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

### std::io::Result

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

### std::io::stdin

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

### std::io::stdout

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

### std::io::Stdout::flush

- Description

  この出力ストリームをフラッシュして、一時的にバッファリングされたコンテンツがすべて目的地に到達するようにします。



---

### std::io::Read::read

- Description

  このソースからいくつかのバイトを指定されたバッファに引き込み、何バイト読まれたかを返します。

  この関数は、データの待ち受けをブロックするかどうかについては何の保証もしませんが、オブジェクトが読み込みのためにブロックする必要があり、ブロックできない場合は、通常は`Err`返り値を介してその旨を通知します。

  このメソッドの戻り値が Ok(n) である場合、`0 <= n <= buf.len()`であることが保証されなければなりません。ゼロでない`n`の値は、バッファ`buf `がこのソースからの`n`バイトのデータで埋め尽くされたことを示します。`n`が 0 の場合は、2 つのシナリオのうちの 1 つを示します。

---

### std::convert::AsRef

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

### std::path::Path

  - Discription

    パスのスライス。

    この型はパスを検査するための多くの操作をサポートしている。パスをその構成要素に分割したり(Unixでは/で区切って、Windowsでは/または/で区切って)、ファイル名を抽出したり、パスが絶対パスかどうかを判断したりなど。

    非サイズ型であり、常に 参照や`Box`のようなポインタの後ろで使用されなければならない。

---

### std::path::Path::new

- Description

  文字列スライスを`Path`スライスとして直接ラップします。

  これはコストのかからない変換です。

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

### std::fs::File::metadata

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

### std::fs::File::open

  - Description

    読み取り専用でファイルを開く。

    この関数は、パスが既に存在しない場合にエラーを返す。

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

### std::fs::OpenOptions::new

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

### std::fs::OpenOptions::read

- Description

  読み込みアクセスのオプションを設定します。

  このオプションが`true`の場合、ファイルを開いたときに読み取り可能であることを示します。

- Example

  ```rust
  use std::fs::OpenOptions;
  
  let file = OpenOptions::new().read(true).open("foo.txt");
  ```



---

### std::fs::OpenOptions::write

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

### std::fs::OpenOptions::create

- Description

  新しいファイルを作成するか、既に存在している場合はそれを開くかのオプションを設定します。

  ファイルを作成するためには、`OpenOptions::write`または `OpenOptions::append`アクセスが使用されなければなりません。

- Example

  ```rust
  use std::fs::OpenOptions;
  
  let file = OpenOptions::new().write(true).create(true).open("foo.txt");
  ```



---

### std::fs::OpenOptions::open

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

### std::fs::write

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

### std::io::Write

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

### std::io::BufRead::lines

  - Description

    Readerの行のイテレータを返す。

    この関数から返されるイテレータは、io::Result<[String]>のインスタンスを返します。返される各文字列は、最後に改行バイト（0xAバイト）やCRLF（0xD、0xAバイト）は持たない。



---

### std::env::Args

  - Description

    プロセスの引数に対するイテレータで、各引数の String 値を返す。
    この構造体は`env::args()`によって作成される。詳細はドキュメントを参照のこと。
    最初の要素は伝統的に実行ファイルのパスですが、任意のテキストを設定することもでき、存在しない場合もある。つまり、このプロパティはセキュリティのために頼るべきではないということである。

---

### std::env::args

  - Description

    このプログラムが開始されたときの引数を返す（通常はコマンドライン経由で渡される）

---

### std::env::set_current_dir

- Description

  現在の作業ディレクトリを、指定したパスに変更します。

  操作に失敗した場合は、`Err`を返します。

---

### std::str::from_utf8

- Description

  バイトのスライスを文字列のスライスに変換します。

  文字列スライス(`&str`)はバイト(`[u8]`)でできており、バイトスライス(`&[u8]`)はバイトでできているので、この関数は両者を変換します。`from_utf8()`は、バイトが有効なUTF-8であることを確認してから変換を行います。

  バイトスライスが有効な UTF-8 であることが確実で、有効性チェックのオーバーヘッドを発生させたくない場合は、この関数の安全ではないバージョンの`[from_utf8_unchecked]`があります。

  `str`の代わりに`String`が必要な場合は、`String::from_utf8`を検討してください。

  `[u8; N]`をスタック割り当てでき、その`&[u8]`を取ることができるので、この関数はスタック割り当てされた文字列を持つ一つの方法です。以下の例のセクションに例があります。

---

### String::from_utf8_lossy

- Description

  無効な文字を含むバイトのスライスを文字列に変換します。

  文字列はバイト([u8])でできており、バイトのスライス(&[u8])はバイトでできているので、この関数は両者を変換します。ただし、すべてのバイトスライスが有効な文字列であるわけではありません: 文字列は有効なUTF-8である必要があります。この変換の際、`from_utf8_lossy()`は無効な UTF-8 シーケンスを`U+FFFD REPLACEMENT CHARACTER`で置き換えます。

  バイトスライスが有効なUTF-8であることが確実で、変換のオーバーヘッドを発生させたくない場合は、この関数の安全でないバージョンである`from_utf8_unchecked`があります。

  この関数は`Cow<'a, str>`を返します。バイトスライスが無効なUTF-8であれば、置換文字を挿入する必要がありますが、これは文字列のサイズを変えることになるので、Stringが必要になります。しかし、すでに有効なUTF-8であれば、新しい割り当ては必要ありません。この戻り値型は、両方のケースを処理することができます。

---

### std::borrow::ToOwned

- Description

  通常は、クローンを作成することによって、借用したデータから所有データを作成します。

---

### std::cmp::Eq

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

### std::cmp::Ord

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

### std::cmp::Ord::max

- Description

  2 つの値の最大値を比較して返します。

  比較の結果、2つの値が等しいと判断された場合は、2番目の引数を返します。

- Example

  ```rust
  assert_eq!(2, 1.max(2));
  assert_eq!(2, 2.max(2));
  ```



---

### std::cmp::Ord::min

- Description

  2 つの値の最小値を比較して返します。

  比較の結果、2 つの値が等しいと判断された場合は、最初の引数を返します。

- Example

  ```rust
  assert_eq!(1, 1.min(2));
  assert_eq!(2, 2.min(2));
  ```



---

### std::cmp::Ord::cmp

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

### std::cmp::Ordering::reverse

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

### std::cmp::PartialEq

  - Description

    半同値関係にある等式の比較を行うトレイト。

    このトレイトは、完全な等価関係を持たない型に対して、部分的な等価関係を可能にします。

    例えば、浮動小数点では`Nan! = Nan`なのでPartialEqを実装していますが、Eqは実装していない。

    形式的には、(すべてのa,b,cに対して)等価でなければならない。

    対称的： a == b は b == a を意味する。
    推移的: a == b と b == c は a == c を意味する。
    これらの要件は、トレイト自体が対称的かつ推移的に実装されなければならないことを意味していることに注意すること。

    もし T: PartialEq<U> と  U: PartialEq<V> の場合、 U: PartialEq<T> と T: PartialEq<V>となる。

    このトレイトは、#[derive]と一緒に使うことができる。構造体で導出された場合、すべてのフィールドが等しい場合は2つのインスタンスが等しく、いずれかのフィールドが等しくない場合は等しくない。enumsで導出された場合、各バリアントはそれ自身と等しく、他のバリアントとは等しくない。

    PartialEq は eq メソッドを実装する必要があるだけで、PartialEqはeqメソッドを使用して定義されている。

    を手動で実装する場合は、eq は PartialEq  の厳密な逆数であるというルールを守らなければならない。

    つまり、`a != b`の場合に限り`!(a==b)`

    PartialEq、PartialOrd、Ord の実装は互いに一致していなけならない。いくつかのトレイトを導出し、他のトレイトを手動で実装することで、誤ってこれらのトレイトを一致させてしまうことは簡単である。

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

### std::cmp::PartialOrd

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

### std::default::Default

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

### std::fmt::Debug

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

    　◆手動で実装

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

### std::iter::FromIterator

  - Description

    型に対して FromIterator を実装することで、イテレータからどのように生成されるかを定義する。

    FromIterator::from_iter() が明示的にコールされることはほとんどなく、代わりに Iterator::collect() メソッドを使用する(詳細は、[Iterator::collect()](https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.collect)を参照)

---

###  std::iter::Iterator::collect

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

### std::iter::Iterator::filter_map

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

### std::iter::Iterator::partition

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

### std::iter::Iterator::any

- Description

  イテレータのいずれかの要素が述語にマッチするかどうかをテストします。

  `any()`は、`true`または`false`を返すクロージャを受け取ります。このクロージャをイテレータの各要素に適用し、それらのうちのどれかが true を返す場合は`any()`も同様です。すべてが `false`を返す場合は`false`を返します。

  つまり、他に何が起こっても結果は真であることを考えると、真を見つけるとすぐに処理を停止します。

  空のイテレータは`false`を返します。

---

### std::iter::Iterator::find

  - Description

---

### std::iter::Iterator::find_map

  - Description

    イテレータの要素に関数を適用して、最初の`non-none`でない結果を返します。

    `iter.find_map(f)`は `iter.filter_map(f).next()`と同等です。

---

### std::iter::Iterator::position

  - Description

    イテレータ内の要素を検索し、そのインデックスを返します。

    `position()`は、`true`または`false`を返すクロージャを受け取ります。このクロージャをイテレータの各要素に適用し、そのうちの 1 つが真を返す場合、 `position()`は`Some(index)`を返します。すべてが `false`を返す場合は `None`を返します。

    `position()`は短絡的な処理を行っています。

---

### std::iter::Iterator::rposition

  - Description

    イテレータ内の要素を右から探し、そのインデックスを返します。

    `rposition()`は、`true`または`false`を返すクロージャを受け取ります。このクロージャをイテレータの各要素に、端から順に適用し、そのうちの1つが真を返すならば、`rposition()`は``Some(index)`を返します。すべてが`false`を返す場合は `None`を返します。

    `rposition()`は短絡的です。言い換えれば、真を見つけるとすぐに処理を停止します。

---

### std::iter::Iterator::next

  - Description

    イテレータを進めて次の値を返す。反復が終了すると [None] を返す。個々のイテレータの実装は、反復処理を再開することを選択することができる。

---

### std::iter::Iterator::take

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

### std::iter::Iterator::skip

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

### std::iter::Iterator::filter

  - Description

    クロージャを使用して要素を生成するかどうかを決定するイテレータを作成する。
    要素が与えられると、クロージャは`true`または`false`を返さなければならない。返されるイテレータは、クロージャが`true`を返す要素のみを返す。

---

### std::iter::Iterator::map

  - Description
    クロージャを受け取り、各要素上でそのクロージャを呼び出すイテレータを作成します。
    `map()`は、引数である`FnMut`を実装したものを使って、あるイテレータを別のイテレータに変換します。これは、元のイテレータの各要素に対してこのクロージャを呼び出す新しいイテレータを生成します。
    型で考えるのが得意な人は、`map()`をこのように考えることができます。ある型`A`の要素を与えるイテレータがあり、他の型`B`のイテレータが欲しい場合は`map()`を使用し、`A`を受け取り`B`を返すクロージャを渡すことができます。
    `map()`は、概念的には`for`ループに似ています。しかし、 map() は怠惰なので、すでに他のイテレータを使用している場合に使用するのが最適です。副次的な効果のために何らかのループを行う場合は、`map()`よりも`for`を使用した方が慣用的だと考えられています。

---

### std::iter::Iterator::take_while

  - Description
    述語に基づいて要素を生成するイテレータを作成します。
    `take_while()`はクロージャを引数に取ります。これは、イテレータの各要素でこのクロージャを呼び出し、それが真を返す間に要素を生成します。
    `false`が返された後、`take_while()`の作業は終了し、残りの要素は無視されます。

---

### std::iter::Iterator::filter

  - Description
    クロージャを使用して要素を生成するかどうかを決定するイテレータを作成します。
    要素が与えられると、クロージャは`true`または`false`を返さなければなりません。返されるイテレータは、クロージャが`true`を返す要素のみを返します。

---

### std::iter::Iterator::fold

  - Description
    関数を適用し、単一の最終値を生成するイテレータメソッド。
    `fold()`は、2つの引数を取ります: 初期値と、2つの引数を持つクロージャ: 'アキュムレータ' と要素です。クロージャは、アキュムレータが次の反復のために持つべき値を返します。
    初期値は、アキュムレータが最初の呼び出しで持つ値です。
    このクロージャをイテレータの各要素に適用した後、`fold()`はアキュムレータを返します。
    この操作は'reduce'や'inject'と呼ばれることもあります。
    折りたたみは、何かのコレクションを持っていて、そこから単一の値を生成したいときに便  利です。
    注意:`fold()`や、イテレータ全体を横断する同様のメソッドは、結果が有限時間内に決定可能な   トレイトあって   も、無限のイテレータでは終了しないことがあります。

---

### std::iter::Iterator::enumerate

- Description

  現在の反復回数と次の値を与えるイテレータを作成します。

  返されるイテレータは、ペア`(i, val)`を返します。

  `enumerate()`は、そのカウントを usize として保持します。異なるサイズの整数でカウントしたい場合は、`zip`関数も同様の機能を提供します。

---

### std::iter::Iterator::all

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

### std::iter::Iterator::peekable

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

### std::iter::Iterator::peek

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

### std::iter::Iterator::rev

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

### std::iter::Iterator::nth

  - Description

    イテレータの n 番目の要素を返す。

    ほとんどのインデックス操作と同様に、カウントはゼロから始まるので、 `nth(0)`は最初の値を返し、 `nth(1)`は 2 番目の値を返す。

    返された要素と同様に、先行するすべての要素がイテレータから消費されることに注意すること。つまり、先行する要素は破棄され、同じイテレータで `nth(0)`を複数回呼び出すと、異なる要素が返されることになる。

    `nth()`は、n がイテレータの長さ以上であれば `None`を返す。

    

---

### std::iter::IntoIterator

- Description

  イテレータへの変換。

  型に対して`IntoIterator`を実装することで、それをイテレータに変換する方法を定義します。これは、ある種のコレクションを記述する型ではよくあることです。

  `IntoIterator`を実装する利点のひとつは、あなたの型が`Rust`の`for loop`構文で動作することです。

---

### std::vec::IntoIter

- Description

  ベクトルの外に移動するイテレータ。

  この構造体は、`[Vec]`の`into_iter`メソッドによって作成されます（`[IntoIterator] trait`によって提供されます）。
  
  

---

### std::vec::Vec::extend_from_slice

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

### std::ops::Add

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

### read_to_string()

  - Description

    End of Fileまですべてのバイトを読み込みバッファに追加する。

    成功した場合、この関数は読み込んでバッファに追加したバイト数を返却する。

    このストリーム内のデータが有効なUTF-8でない場合、エラーを返却し、バッファに追加されない。

    そのほかの、エラーセマンティクスは[read_to_end](https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_end)を参照のこと。

---

### std::process

  - Description

    プロセスを扱うモジュール。

    このモジュールは、主に子プロセスの生成と相互作用に関係していますが、現在のプロセスを終了させるための`abort`と `exit]`も提供している。

---

### std::process::Command

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

### std::process::Command::spawn

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

### std::process::exit

  - Description

    指定した終了コードで現在のプロセスを終了させる。

    この関数は何も返却せず、現在のプロセスを即座に終了させる。終了コードは基盤となるOSに渡され、別のプロセスで使用できるようになる。

    この関数は何も返却せず、プロセスを終了するので、現在のスタックや他のスレッドのスタック上のデストラクタは実行されないことに注意すること。クリーンなシャットダウンが必要な場合は、実行するデストラクタがなくなった時点でのみこの関数を呼び出すことを検討すること。

---

### std::process::Child

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

### str::contains

  - Description

    与えられたパターンが、この文字列スライスのサブスライスにマッチした場合に真を返す。
    そうでない場合は false を返す。
    パターンには、`&str`、`char`、文字列のスライス、文字がマッチするかどうかを判定する関数やクロージャを指定することができる。

---

### str::chars

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

### str::split_whitespace

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

### str::lines

  - Description

    各行の文字列を文字列スライスとして、イテレータを返す。

    行は、改行（\ n）または改行によるキャリッジリターン（\ r \ n）のいずれかで終了する。

    最終行終了はオプションである。最終行終了で終わる文字列は、最終行終了のない、そうでなければ同一の文字列と同じ行を返す。



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

### slice::windows

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

### slice::contains

- Descriptiom

  スライスに指定した値の要素が含まれている場合はtrueを返します。

---

### slice::starts_with

- Description

  needleがスライスの接頭辞である場合にtrueを返します。

---

### slice::ends_with

- Description

  needleがスライスの接尾辞である場合にtrueを返します。



---

### slice::sort

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

### slice::sort_by

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

### slice::iter_mut

- Description

  各値を変更できるようにするイテレータを返します。



---

### slice::iter

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

### std::vec::Vec::dedup

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

### std::vec::Vec::with_capacity

- Description

  指定された容量の新しい空の`Vec<T>`を作成します。

  `Vec`は、再割り当てを行わずに正確に容量要素を保持することができます。capacityが0の場合、ベクタは割り当てを行いません。

  返されたベクタは指定された容量を持っていますが、ベクタの長さはゼロになることに注意することが重要です。長さと容量の違いについての説明は、容量と再割り当てを参照してください。

  

  

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

### str::to_lowercase

  - Description

    この文字列スライスの小文字に相当するものを、新しい [String] として返す。
    `Lowercase`は、Unicode Derived Core Property Lowercaseの条項に従って定義される。
    大文字小文字を変更すると複数の文字に展開されてしまう文字があるため、この関数はパラメータをそのまま変更するのではなく、[String]として返す。



---

### char::to_digit

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

### std::any::TypeId

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

### std::marker::PhantomData

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

### std::env::var

  - Description

    現在のプロセスから環境変数のキーを取得する。




---

### std::time::Duration

  - Description

    Durationは、通常システムのタイムアウトに使用される時間のスパンを表す。
    各Durationは、秒の整数とナノ秒で表される端数で構成される。基礎となるシステムがナノ秒レベルの精度をサポートしていない場合、システム タイムアウトをバインディングするAPIは通常、ナノ秒数を切り上げる。
    Durationは、Add、Sub、その他のopsトレイトなど、多くの一般的なトレイトを実装している。長さ0のDurationを返すことでDefaultを実装している。

---

### std::time::Instant

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

### `std::hash::Hash`

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

### `std::hash::Hasher`

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

### std::collections::HashMap

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

  キーがマップの中にある間に、Hash形質によって決定されるキーのハッシュや、`Eq`トレイトによって決定されるキーの等値性が変化するような方法でキーが修正されることは、論理エラーです。これは通常、`Cell`、`RefCell`、グローバルステート、I/O、または安全でないコードによってのみ可能です。このような論理エラーから生じる動作は規定されていませんが、未定義の動作になることはありません。これには、パニック、不正な結果、アボート、メモリリーク、終了しないことなどが含まれます。

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

### std::collections::hash_map::HashMap::insert

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

### std::collections::hash_map::HashMap::get

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

### std::collections::hash_map::HashMap::remove

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

### std::collections::HaseSet

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

### std::collections::hash_set::union

- Description

  結合を表す値、つまりselfやotherのすべての値を、重複することなくアクセスします。

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

### std::collections::hase_set::contains

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

### std::collections::hase_set::intersection

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

### std::collections::hase_set::difference

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

### std::thread

  - Description

    - The threading model
      実行中のRustプログラムは、ネイティブOSのスレッドのコレクションで構成されており、それぞれが独自のスタックとローカルステートを持っている。
      スレッドは名前を付けることができ、低レベルの同期のためのいくつかの組み込みサポートを提供している。
      スレッド間の通信は、チャンネル、Rust のメッセージ通過型、他の形式のスレッド同期、共有メモリデータ構造を介して行うことができる。特に、スレッドセーフであることが保証されている型は、原子的に参照カウントされたコンテナ`Arc`を使用してスレッド間で簡単に共有することができる。
      Rust で致命的なロジックエラーが発生すると、スレッドパニックが発生し、その間にスレッドはスタックを巻き戻し、デストラクタを実行し、所有するリソースを解放します。これは、スレッドがスタックを巻き戻し、デストラクタを実行し、所有するリソースを解放することを意味する。パニックが捕捉されなかった場合、スレッドは終了しますが、オプションで`join`を使用して別のスレッドからパニックを検出することができる。パニックが捕捉されずにメインスレッドがパニックに陥った場合、アプリケーションは0以外の終了コードで終了します。
      Rust プログラムのメインスレッドが終了すると、他のスレッドが実行中であってもプログラム全体がシャットダウンします。しかし、このモジュールは自動的に子スレッドの終了を待つための便利な機能を提供している。
    - Spawning a thread
      新しいスレッドは`thread::spawn`関数を使って生成することができる。

    ~~~rust
    use std::thread;
    
    thread::spawn(move || {)
        // some work here
    });
    ~~~

    この例では、スポーンされたスレッドは現在のスレッドから「切り離された」状態になっている。これは、この親がメインスレッドでない限り、その親 (それをスポーンしたスレッド) よりも長生きできることを意味する。
    親スレッドは、子スレッドの完了を待つこともできます。`spawn`の呼び出しでは、待ちのための`join`メソッドを提供する`JoinHandle`が生成されます。

    

    ~~~rust
    use std::thread;
    
    let child = thread::spawn(move || {
        // some work here
    });
    // some work here
    let res = child.join();
    ~~~

    		joinメソッドは、子スレッドが生成した最終的な値のOkを含む`thread::Result`を返し、子スレッドがパニ		ックに陥った場合は`panic!`コールに与えられた値のErrを返す。

    - Configuring threads
      新しいスレッドは、ビルダータイプを介してスポーンされる前に設定することができ、現在のところ子スレッドの名前とスタックサイズを設定することができる。

    ~~~rust
    use std::thread;
    
    thread::Builder::new().name("child1".to_string()).spawn(move || {
        println!("Hello, world!");
    });
    ~~~

    - Spawning a thread

      スレッドは、2 つの方法のいずれかで取得できる `Thread`型を介して表現される。

      - `thread::current`関数を使用して、現在のスレッドを要求する。
      - `thread::current`関数は、このモジュールの`API`によってスポーンされないスレッドに対しても利用可能。

    - Thread-local storage

      このモジュールは、`Rust`プログラム用のスレッドローカルストレージの実装も提供する。スレッドローカルストレージは、プログラム内の各スレッドが独自のコピーを持つグローバル変数にデータを格納する方法。スレッドはこのデータを共有しないので、アクセスを同期させる必要はない。スレッドローカルキーは、それが含む値を所有し、スレッドが終了したときにその値を破棄します。thread_local! マクロで作成され、`'static`(借用ポインタはない)な値を含むことができる。これは、指定されたクロージャへの値への共有参照を生成するアクセサ関数`with`を提供します。スレッドローカルキーは、値への共有アクセスのみを許可している。ほとんどの値は、`Cell`型や`RefCell`型を用いて何らかの形式の内部変異性を利用したいと考えるであろう。

    - Naming threads

      スレッドは、識別のために関連付けられた名前を持つことができる。デフォルトでは、スポーンされたスレッドには名前が付けられていない。スレッドの名前を指定するには、`Builder`でスレッドをビルドし、希望するスレッド名を`Builder::name`に渡します。スレッド名をスレッド内から取得するには Thread::name を使用します。スレッド名が使用される例をいくつか挙げます。

      - 指定されたスレッドでパニックが発生した場合、そのスレッド名がパニックメッセージに表示される。
      - スレッド名は、該当する場合には OS に提供される (例: unix ライクなプラットフォームでは `pthread_setname_np`)。

    - Stack size

      スポーンされるスレッドのデフォルトのスタックサイズは 2 MiB ですが、この特定のスタックサイズは将来的に変更される可能性がある。スポーンされるスレッドのスタックサイズを手動で指定するには、2つの方法がある。

      - Builder でスレッドをビルドし、希望するスタックサイズを`Builder::stack_size`に渡す。
      - 環境変数`RUST_MIN_STACK`を、希望するスタックサイズを表す整数 (バイト単位)に設定する。`Builder::stack_size`の設定はこれをオーバーライドすることに注意。

      メインスレッドのスタックサイズは Rust によって決定されないことに注意。

    

---

### std::thread::spawn

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

### [std](https://doc.rust-lang.org/stable/std/index.html)::[thread](https://doc.rust-lang.org/stable/std/thread/index.html)::JoinHandle

- Description

  スレッドに参加するために所有されているパーミッション(終了時にブロック)。

  `JoinHandle`は、関連するスレッドが削除されたときに関連するスレッドを切り離します。

  プラットフォームの制限のため、このハンドルを複製することはできません: スレッドに参加する能力は、一意に所有する権限です。

  この構造体は `thread::spawn`関数と`thread::Builder::spawn`メソッドによって作成されます。

---

### std::thread::JoinHandle::join

  - Description

    関連するスレッドの終了を待つ。
    アトミックメモリの順序付けの観点からは、関連付けられたスレッドの完了は、この関数のリターンと同期する。
    言い換えれば、そのスレッドによって実行されたすべての操作は、joinが戻ってきた後に発生するすべての操作よりも先に順序付けられる。
    子スレッドがパニックに陥った場合、`panic！`に与えられたパラメータで`Err`が返される。
    この関数は、プラットフォームによってはスレッドが自分自身に参加しようとした場合にパニックを起こすかもしれないし、そうでなければスレッドの`join`でデッドロックを起こすかもしれない。

---

### std::thread::sleep

  - Description

    現在のスレッドを、少なくとも指定した時間だけスリープ状態にする。
    スレッドは、スケジューリングの仕様やプラットフォーム依存の機能のために、指定された時間よりも長くスリープすることがある。スレッドのスリープ時間が短くなることはない。
    この関数はブロッキングであり、非同期関数では使用すべきではない。

---

### std::sync::mpsc

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

### mpsc::channel

  - Description

    新しい非同期チャンネルを作成し、送信者と受信者の半分を返す。`Sender`で送信されたすべてのデータは、送信された順に `Receiver`で利用可能になり、`send`が呼び出し元のスレッドをブロックすることはない(このチャンネルには "無限のバッファ" があり、バッファの限界に達するとブロックされる `sync_channel`とは異なる)。
    `Sender` をクローンして同じチャンネルに複数回送信することができますが、サポートされているのは 1 つの`Receiver`のみ。

    `Sender`で送信しようとしている間に`Receiver`が切断された場合、送信メソッドは `SendError`を返す。同様に、`Sender`が`recv`しようとしているときに切断された場合、`recv`メソッドは`RecvError`を返す。

---

### std::sync::mpsc::Sender::send

  - Description

    このチャネルに値を送信しようとし、送信できなかった場合は値を返す。
    送信が成功した場合は、チャンネルの相手側ハングアップしていないと判断された場合。送信に失敗した場合は、対応するチャンネルが既に割り当て解除されている場合。`Err`の戻り値はデータを受信しないことを意味し、`Ok`の戻り値はデータを受信することを意味しないことに注意すること。この関数が`Ok`を返した直後に、対応するチャンネルがハングアップする可能性がある。
    このメソッドは、現在のスレッドをブロックすることはない。

---

### std::sync::mpsc::Receive::recv

  - Description

    この受信機で値の待ち受けを試み、対応するチャンネルがハングアップした場合はエラーを返す。
    この関数は、利用可能なデータがなく、より多くのデータを送信できる可能性がある場合、常に現在のスレッドをブロックする。対応する`Sender`(または`SyncSender`) にメッセージが送信されると、このレシーバはウェイクアップしてそのメッセージを返す。
    対応する`Sender`が切断された場合や、このコールがブロックされている間に切断された場合は、このコールはウェイクアップして`Err`を返し、このチャンネルではこれ以上メッセージを受信できないことを示す。ただし、チャネルはバッファリングされているので、切断前に送信されたメッセージは正しく受信される。

---

### std::sync::Mutex

  - Description

  共有データの保護に有用な相互排除プリミティブ
  このmutexは、ロックが利用可能になるのを待つスレッドをブロックする。`mutex`は静的に初期化したり、新しいコンストラクタを使って作成することもできます。各`mutex`には保護するデータを表す`type`パラメータがあります。データは`lock`と`try_lock`から返される RAII ガードを介してのみアクセスでき、`mutex`がロックされているときにのみデータにアクセスできることを保証する。

  - Poisoning
    このモジュールのmutexは「Poisoning」と呼ばれる戦略を実装しており、mutexを保持している間にスレッドがパニックになると、いつでもmutexがポイズニングされているとみなされます。一度mutexがポイズニングされると、他のすべてのスレッドはデータが汚染されている可能性が高いので、デフォルトではデータにアクセスできなくなります(何らかの不変量が保持されていない)。
    mutexの場合、これは`lock`メソッドと`try_lock`メソッドが、`mutex`がポイズンされたかどうかを示す `Result`を返すことを意味します。mutexのほとんどの使用法では、これらの結果を単に unwrap() して、無効な不変量が目撃されないようにスレッド間でパニックを伝播させる。
    しかし、ポイズンされたmutexは、基礎となるデータへのすべてのアクセスを妨げるものではない。`PoisonError`型には`into_inner`メソッドがあり、これはロックが成功したときに返されるはずのガードを返す。これにより、ロックがポイズンされているにもかかわらず、データへのアクセスが可能になる。



---

### std::sync::Mutex::lock

  - Description

    mutexを取得し、それが可能になるまで現在のスレッドをブロックします。

    この関数は、mutexを取得できるようになるまでローカルスレッドをブロックします。復帰時には、そのスレッドはロックが保持されている唯一のスレッドとなります。ロックのスコープ付きアンロックを可能にするために、`RAII`ガードが返されます。ガードがスコープ外になると、mutexはアンロックされる。
    既にロックを保持しているスレッドでmutexをロックする場合の正確な動作は未定義である。しかし、この関数は2回目の呼び出しでは戻りません(例えば、パニックやデッドロックになる可能性がある)。

    - Error
      このmutexを保持している間にこのmutexの他のユーザがパニックに陥った場合、この呼び出しはmutexを取得した後にエラーを返す。

    - Panic
      この関数は、現在のスレッドが既にロックを保持している場合に呼び出されるとパニックになる可能性がある。

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

### std::rc::Rc

- Description

  シングルスレッドの参照カウント式ポインター。`Rc`は「Reference Counted」の略です。

  詳細は、モジュールレベルのドキュメントを参照してください。

  `Rc`の固有のメソッドはすべて関連する関数であり、`value.get_mut()`ではなく、例えば`Rc::get_mut(&mut value)`のように呼ばなければならないことを意味します。これにより、内部型Tのメソッドとの衝突を避けることができます。

---

### std::rc::Rc::strong_count

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

### std::rc::Rc::clone

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

### std::sync::Arc

  - Description

    スレッドセーフな参照カウントポインタ。`Arc`は`Atomically Reference Counted`の略。

    `Arc<T>`型は、ヒープに割り当てられた`T`型の値の共有所有権を提供する。`Arc`上で`clone`を実行すると、参照カウントを増加させながら、ソース`Arc`と同じヒープ上の割り当てを指す新しい`Arc`インスタンスが生成される。与えられたアロケーションへの最後の Arc ポインタが破棄されると、そのアロケーションに格納されている値 (多くの場合、「内部値」と呼ばれます) も削除されます。

    Rust の共有参照はデフォルトで突然変異を禁止しており、Arc も例外ではありません。Arc を通してミューテーションを行う必要がある場合は、Mutex、RwLock、または Atomic 型のいずれかを使用してください。

    Rc<T> とは異なり、Arc<T> は参照カウントにアトミック演算を使用します。これはスレッドセーフであることを意味します。欠点は、アトミック演算が通常のメモリアクセスに比べて高価なことです。スレッド間で参照カウントされた割り当てを共有しない場合は、より低いオーバーヘッドのために Rc<T> の使用を検討してください。スレッド間で Rc<T> を送ろうとすると、コンパイラがそれをキャッチするので、`Rc<T>`は安全なデフォルトです。しかし、ライブラリの利用者に柔軟性を持たせるために、ライブラリは`Arc<T>`を選択するかもしれません。

    `Arc<T>`は、`T`が`Send`と`Sync`を実装している限り、`Send`と`Sync`をする。スレッドセーフではない型の`T`を`Arc<T>`に入れてスレッドセーフにすることができないのはなぜか？最初は少し直観的ではないかもしれないが、結局のところ、`Arc<T>` のスレッドセーフは重要ではないのではないのか？結局のところ、`Arc<T>`のスレッド安全性は重要ではないのではないのか？重要なのは、`Arc<T>`は、同じデータの複数の所有権を持つことをスレッドセーフにする、そのデータにスレッドセーフを追加するわけではない。`Arc<RefCell<T>>`を考えてみる。RefCell<T>はSyncではないので、もしArc<T>が常にSendであれば、Arc<RefCell<T>も同様に`Send`になります。しかし、そうすると問題が発生する。`RefCell<T>`はスレッドセーフではない。

    `RefCell<T>`はスレッドセーフではないので、非アトミック演算を使って借用回数を追跡する。

    `downgrade`メソッドを使用して、所有権のない`Weak`ポインタを作成することができる。`Weak`ポインタを`Arc`にアップグレードすることができますが、アロケーションに格納されている値が既にドロップされている場合は`None`を返す。言い換えれば、`Weak` `ポインタはアロケーション内の値を保持しませんが、アロケーション (値の裏付けとなるストア) を保持する。

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
      `Arc<T>`は自動的に (Deref trait を介して) `T`に派生するので、`Arc<T>`型の値に対して`T`のメソッドを呼び出すことができる。`T`のメソッドとの名前の衝突を避けるため、`Arc<T>`のメソッドは関連する関数であり、完全修飾構文を用いて呼び出される。

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

### std::sync::atomic

- Description

  Atomic型

  アトミック型はスレッド間のプリミティブな共有メモリ通信を提供し、他の並行型の構成要素となります。
  このモジュールは、`AtomicBool`、`AtomicIsize`、`AtomicUsize`、`AtomicI8`、`AtomicU16`などを含む、選択された数のAtomic型のアトミックバージョンを定義します。Atomic型は、正しく使用されるとスレッド間の更新を同期させる操作を提供します。
  各メソッドは、その操作のためのメモリバリアの強さを表す順序を取ります。これらの順序付けは、C++20 のアトミック順序付けと同じです。詳細については、[nomicon](https://doc.rust-lang.org/stable/nomicon/atomics.html)を参照してください。
  Atomic変数はスレッド間で共有しても安全ですが（`Sync`を実装しています）、それ自体は共有のメカニズムを提供しておらず、Rustのスレッドモデルに従っています。アトミック変数を共有する最も一般的な方法は、`Arc`(原子的に参照カウントされた共有ポインタ) に格納することです。
  Atomic型は静的変数に格納され、`AtomicBool::new`のような定数初期化子を使って初期化されます。Atomic静的変数は、遅延グローバル初期化によく使われます。

---

### std::net::TcpListener

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

### std::net::TcpStream

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