### クロージャ

- 基本構文

```rust
let plus_one = |x: i32| x + 1;

assert_eq!(2, plus_one(1));
```

クロージャの引数はパイプ( `|` )の間に書く。そしてクロージャの本体は式なので、 この場合は `x + 1` がそれに当たる。 `{ }` が式であることを思い出して下さい、 そのため複数行のクロージャを作成することも可能。

```rust
let plus_two = |x| {
    let mut result: i32 = x;

    result += 1;
    result += 1;

    result
};

assert_eq!(4, plus_two(2));
```
クロージャは、通常の`fn`で定義される関数と異なる。

一つ目に、引数や返り値の型を示す必要がないことです。
```rust
let plus_one = |x: i32| -> i32 { x + 1 };

assert_eq!(2, plus_one(1));
```

二つ目に、構文は非常に似ていますが、ほんの少しだけ異なるという点
```rust
fn  plus_one_v1   (x: i32) -> i32 { x + 1 }
let plus_one_v2 = |x: i32| -> i32 { x + 1 };
let plus_one_v3 = |x: i32|          x + 1  ;
```

- クロージャとクロージャの環境

クロージャの環境は引数やローカルな束縛に加えてクロージャを囲んでいるスコープ中の束縛を含むことができます。 例えば以下のようになります:

```rust
let num = 5;
let plus_num = |x: i32| x + num;

assert_eq!(10, plus_num(5));
```

クロージャ `plus_num` はスコープ内の  `num` を参照しています。 より厳密に言うと、クロージャ `plus_num` は`num`を借用しています。 もし、この束縛と衝突する処理を行うとエラーが発生します。 例えば、以下のようなコードでは:

```rust
let mut num = 5;
let plus_num = |x: i32| x + num;

let y = &mut num;
```

以下のエラーを発生させます:

```
error: cannot borrow `num` as mutable because it is also borrowed as immutable
    let y = &mut num;
                 ^~~
note: previous borrow of `num` occurs here due to use in closure; the immutable
  borrow prevents subsequent moves or mutable borrows of `num` until the borrow
  ends
    let plus_num = |x| x + num;
                   ^~~~~~~~~~~
note: previous borrow ends here
fn main() {
    let mut num = 5;
    let plus_num = |x| x + num;

    let y = &mut num;
}
^
```

冗長ですが役に立つエラーメッセージです! エラーが示しているように、クロージャが既に `num` を借用しているために、 `num` の変更可能な借用を取得することはできません。 もしクロージャがスコープ外になるようにした場合以下のようにできます:

```rust
let mut num = 5;
{
    let plus_num = |x: i32| x + num;

# // } // plus_num goes out of scope, borrow of num ends
} // plus_numがスコープ外に出て、numの借用が終わります

let y = &mut num;
```

もしクロージャが `num` を要求した場合、Rustは借用する代わりに環境の所有権を取りムーブします。 そのため、以下のコードは動作しません:

```rust
let nums = vec![1, 2, 3];

let takes_nums = || nums;

println!("{:?}", nums);
```

このコードは以下の様なエラーを発生させます:

```
note: `nums` moved into closure environment here because it has type
  `[closure(()) -> collections::vec::Vec<i32>]`, which is non-copyable
let takes_nums = || nums;
                 ^~~~~~~
```

`Vec<T>` はその要素に対する所有権を持っています、 それゆえそれらの要素をクロージャ内で参照した場合、 `nums` の所有権を取ることになります。 これは `nums`を `nums` の所有権を取る関数に渡した場合と同じです。

- `move`キーワード

`move` キーワードを用いることで、クロージャに環境の所有権を取得することを強制することができます。

```rust
let num = 5;

let owns_num = move |x: i32| x + num;
```

このようにすると `move` というキーワードにもかかわらず、変数は通常のmoveのセマンティクスに従います。 この場合、 `5` は `Copy` を実装しています、 そのため `owns_num` は `num` のコピーの所有権を取得します。 では、なにが異なるのでしょうか？

```rust
let mut num = 5;

{
    let mut add_num = |x: i32| num += x;

    add_num(5);
}

assert_eq!(10, num);
```

このケースでは、クロージャは `num` の変更可能な参照を取得し、 `add_num` を呼び出した時、期待通りに `num` の値を変更します。 またクロージャ `add_num` はその環境を変更するため `mut` として宣言する必要があります。

もしクロージャを `move` に変更した場合、結果が異なります:

```rust
let mut num = 5;

{
    let mut add_num = move |x: i32| num += x;

    add_num(5);
}

assert_eq!(5, num);
```

結果は `5` になります。 `num` の変更可能な借用を取得するのではなく、 `num` のコピーの所有権を取得します。

`move` クロージャを捉えるもう一つの観点は: `move` クロージャは独自のスタックフレームを持っているという点です。 `move` クロージャは自己従属していますが、 `move` でないクロージャはクロージャを作成したスタックフレームと紐付いています。 これは一般的に、`move` でないクロージャを関数から返すことはできないということを意味しています。

クロージャを引数や返り値にすることについて説明する間に、クロージャの実装についてもう少し説明する必要があります。 システム言語としてRustはコードの動作についてコントロールする方法を大量に提供しています、 そしてそれはクロージャも例外ではありません。

- クロージャの実装

Rustにおけるクロージャの実装は他の言語とは少し異なります。 Rustにおけるクロージャは実質的にトレイトへの糖衣構文です。

クロージャの内部的な動作を理解するための鍵は少し変わっています: 関数を呼び出すのに `()` を 例えば `foo()` の様に使いますが、この `()` はオーバーロード可能な演算子です。 この事実から残りの全てを正しく理解することができます。 Rustでは、トレイトを演算子のオーバーロードに利用します。 それは関数の呼び出しも例外ではありません。 `()` をオーバーロードするのに利用可能な、3つの異なるトレイトが存在します:

```rust
# mod foo {
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
# }
```

これらのトレイトの間のいくつかの違いに気がつくことでしょう、しかし大きな違いは `self` についてです: `Fn` は `&self` を引数に取ります、 `FnMut` は `&mut self` を引数に取ります、そして `FnOnce` は `self` を引数に取ります。 これは通常のメソッド呼び出しにおける `self` のすべての種類をカバーしています。 しかし、これら `self` の各種類を一つの大きなトレイトにまとめるのではなく異なるトレイトに分けています。 このようにすることで、どのような種類のクロージャを取るのかについて多くをコントロールすることができます。

クロージャの構文 `|| {}` は上述の3つのトレイトへの糖衣構文です。 Rustは環境用の構造体を作成し、 適切なトレイトを `impl` し、それを利用します。

- クロージャを引数に取る

クロージャが実際にはトレイトであることを学んだので、 クロージャを引数としたり返り値としたりする方法を既に知っていることになります: 通常のトレイトと同様に行うのです!

これは、静的ディスパッチと動的ディスパッチを選択することができるということも意味しています。 手始めに呼び出し可能な何かを引数にとり、それを呼び出し、結果を返す関数を書いてみましょう:

```rust
fn call_with_one(c: impl Fn(i32) -> i32) -> i32 {
    c(1)
}

fn main() {
    let answer = call_with_one(|x| x + 2);
    assert_eq!(3, answer);
}
```

`call_with_one`関数は、`i32`を引数に取り、`i32`を返します。そのため、ジェネリックの境界として`Fn(i32) -> i32`を指定している。

キーポイントがほかにもあります: ジェネリックをトレイトで境界を指定したために、 この関数は単相化され、静的ディスパッチをクロージャに対して行います

多くの言語では、クロージャは常にヒープにアロケートされ、常に動的ディスパッチが行われます。 Rustではスタックにクロージャの環境をアロケートし、呼び出しを静的ディスパッチすることができます。 これは、しばしばクロージャを引数として取る、イテレータやそれらのアダプタにおいて頻繁に行われます。

もちろん、動的ディスパッチを行いたいときは、そうすることもできます。 そのような場合もトレイトオブジェクトが通常どおりに対応します:

```rust
fn call_with_one(some_closure: &dyn Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

fn main() {
    let answer = call_with_one(&|x| x + 2);
    assert_eq!(3, answer);
}
```

トレイトオブジェクト `&dyn Fn` を引数にとります。 また `call_with_one` にクロージャを渡すときに参照を利用するようにしました、 そのため `&||` を利用しています。

- 関数ポインタとクロージャ

関数ポインタは環境を持たないクロージャのようなものです。 そのため、クロージャを引数として期待している関数に関数ポインタを渡すことができます。

```rust
fn call_with_one(some_closure: &dyn Fn(i32) -> i32) -> i32 {
    some_closure(1)
}

fn add_one(i: i32) -> i32 {
    i + 1
}

let f = add_one;

let answer = call_with_one(&f);

assert_eq!(2, answer);
```

- クロージャを返す

関数を用いたスタイルのコードでは、クロージャを返すことは非常によく見られます。 もし、クロージャを返すことを試みた場合、エラーが発生します。これは一見奇妙に思われますが、理解することができます。 以下は、関数からクロージャを返すことを試みた場合のコードです

```rust
fn factory() -> (Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

fn main() {
    let f = factory();
    let answer = f(1);
    println!("{}", answer);
}
```

このコードは以下の長いエラーを発生させます

```
error[E0746]: return type cannot have an unboxed trait object
 --> src/main.rs:1:18
  |
1 | fn factory() -> (Fn(i32) -> i32) {
  |                  ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/main.rs:4:5: 4:16]`, which implements `Fn(i32) -> i32`
  |
1 | fn factory() -> (impl Fn(i32) -> i32) {
  |                  ^^^^^^^^^^^^^^^^^^^

error[E0277]: the size for values of type `dyn Fn(i32) -> i32` cannot be known at compilation time
 --> src/main.rs:8:9
  |
8 |     let f = factory();
  |         ^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `dyn Fn(i32) -> i32`
  = note: all local variables must have a statically known size
  = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `(dyn Fn(i32) -> i32 + 'static)` cannot be known at compilation time
 --> src/main.rs:8:13
  |
8 |     let f = factory();
  |             ^^^^^^^ doesn't have a size known at compile-time
  |
  = help: the trait `Sized` is not implemented for `(dyn Fn(i32) -> i32 + 'static)`
  = note: the return type of a function must have a statically known size
```

関数から何かを返すにあたって、Rustは返り値の型のサイズを知る必要があります。 しかし、 `Fn`はトレイトであるため、そのサイズや種類は多岐にわたることになります: 多くの異なる型が`Fn`を実装できます。 何かにサイズを与える簡単な方法は、それに対する参照を取得する方法です、参照は既知のサイズを持っています。 そのため、以下のように書くことができます

```rust
fn factory() -> &(dyn Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

fn main() {
    let f = factory();
    let answer = f(1);
    println!("{}", answer);
}
```

しかし、他のエラーが発生してしまいます:

```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:17
  |
1 | fn factory() -> &(dyn Fn(i32) -> i32) {
  |                 ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
1 | fn factory() -> &'static (dyn Fn(i32) -> i32) {
  |                 ^^^^^^^^
```

これはリファレンスを利用したので、ライフタイムを指定する必要が有るためです。 しかし、 factory() 関数は引数を何も取りません、 そのため ライフタイムの省略 は実施されません。 では、どのような選択肢が有るのでしょうか？ 'static を試してみましょう:

```rust
fn factory() -> &'static (dyn Fn(i32) -> i32) {
    let num = 5;

    |x| x + num
}

fn main() {
    let f = factory();
    let answer = f(1);
    println!("{}", answer);
}
```


```
error[E0308]: mismatched types
 --> src/main.rs:4:5
  |
1 | fn factory() -> &'static (dyn Fn(i32) -> i32) {
  |                 ----------------------------- expected `&'static (dyn Fn(i32) -> i32 + 'static)` because of return type
...
4 |     |x| x + num
  |     ^^^^^^^^^^^
  |     |
  |     expected reference, found closure
  |     help: consider borrowing here: `&|x| x + num`
  |
  = note: expected reference `&'static (dyn Fn(i32) -> i32 + 'static)`
               found closure `[closure@src/main.rs:4:5: 4:16]`
```

このエラーは`&'static Fn(i32) -> i32`ではなく、 `[closure@src/main.rs:4:5: 4:16] `を使ってしまっているということを伝えています。 ちょっと待ってください、一体これはどういう意味でしょう？

それぞれのクロージャはそれぞれの環境用の `struct`を生成し、 `Fn `それに準ずるものを実装するため、それぞれの型は匿名となります。 それらの型はそれらのクロージャのためだけに存在します。 そのためRustはそれらの型を自動生成された名前の代わりに`closure@src/main.rs` と表示します。

また、このエラーは返り値の型が参照であることを期待しているが、 上のコードではそうなっていないということについても指摘しています。 もうちょっというと、直接的に 'static ライフタイムをオブジェクトに割り当てることはできません。 そこで`Fn`をボックス化することで「トレイトオブジェクト」を返すという方法を取ります。 そうすると、動作するまであと一歩のところまで来ます

```rust
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(|x| x + num)
}

fn main() {
    let f = factory();
    let answer = f(1);
    println!("{}", answer);
}
```

最後に残されたエラーは以下のとおりです

```
error[E0308]: mismatched types
 --> src/main.rs:4:5
  |
1 | fn factory() -> Box<dyn Fn(i32) -> i32> {
  |                 ----------------------- expected `Box<(dyn Fn(i32) -> i32 + 'static)>` because of return type
...
4 |     |x| x + num
  |     ^^^^^^^^^^^
  |     |
  |     expected struct `Box`, found closure
  |     help: store this in the heap by calling `Box::new`: `Box::new(|x| x + num)`
  |
  = note: expected struct `Box<(dyn Fn(i32) -> i32 + 'static)>`
```

クロージャはその環境を借用します。 今回の場合は、環境はスタックにアロケートされた`5`に束縛された`num`からできていることから、 環境の借用はスタックフレームと同じライフタイムを持っています。 そのため、もしこのクロージャを返り値とした場合、 そのあと`factory()` 関数の処理は終了し、スタックフレームが取り除かれクロージャはゴミとなったメモリを参照することになります! 上のコードに最後の修正を施すことによって動作させることができるようになります

```rust
fn factory() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    Box::new(move |x| x + num)
}

fn main() {
    let f = factory();
    let answer = f(1);
    println!("{}", answer);
}
```

`factory()`内のクロージャを`move Fn`にすることで、新しいスタックフレームをクロージャのために生成します。 そしてボックス化することによって、既知のサイズとなり、現在のスタックフレームから抜けることが可能になります。

---

### Builderパターン

ユーザが Circle を作成できるようにしつつも、書き換えたいプロパティだけを設定すれば良いようにしたいとしましょう。もし指定が無ければ x と y が 0.0 、 radius が 1.0 であるものとします。Rustはメソッドのオーバーロードや名前付き引数、可変個引数といった機能がない代わりにBuilderパターンを採用しており、それは以下のようになります。

```rust
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder {
            x: 0.0,
            y: 0.0,
            radius: 1.0,
        }
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius }
    }
}

fn main() {
    let c = CircleBuilder::new()
            .x(1.0)
            .y(1.0)
            .radius(2.0)
            .finalize();

    println!("area: {}", c.area());
    println!("x: {}", c.x);
    println!("y: {}", c.y);
}
```

ここではもう1つの `struct` である `CircleBuilder` を作成しています。その中にBuilderメソッドを定義しました。また `Circle` に `area()` メソッドを定義しました。 そして `CircleBuilder` にもう1つ `finalize()` というメソッドを作りました。このメソッドはBuilderから最終的な `Circle` を作成します。さて、先程の要求を実施するために型システムを使いました。 `CircleBuilder` のメソッドを好きなように組み合わせ、作る `Circle` への制約を与えることができます。
