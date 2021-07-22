### 静的ディスパッチ

トレイト境界を使用することで、静的ディスパッチを実行できる。
以下は、引数に`imple Foo`というトレイト境界を指定し、`Foo`トレイトを実装する型を引数にとる関数を実装している。返り値に指定した場合でも、基本的な考え方は同じである。

```rust
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

// `Foo`トレイトを実装した型を引数にとる
fn do_something(x: impl Foo) {
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(x);
    do_something(y);
}
```

---

### 動的ディスパッチ

トレイトオブジェクトは、`Box<dyn Foo>`や`&dyn Foo`のように記述され、指定されたトレイトを実装するあらゆる型を保持する値です。その正確な型は、実行時に判明する。
注意点として、低速な仮想関数の呼び出しが必要となる。


```rust
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn do_something(x: &dyn Foo) {
    x.method();
}

// `Box<dyn Foo>`で記述した場合
// fn do_something(x: Box<dyn Foo>) {
//     x.method();
// }

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    do_something(&x);
    do_something(&y);

    // `Box<dyn Foo>`の場合は、以下のように記述する
    //do_something(Box::new(x));
    //do_something(Box::new(x));
}
```

---

### トレイトオブジェクトの内部表現

トレイトのメソッドはトレイトオブジェクト内にある伝統的に「vtable」（これはコンパイラによって作成、管理されます）と呼ばれる特別な関数ポインタのレコードを介して呼び出されます。

トレイトオブジェクトは単純ですが難解でもあります。 核となる表現と設計は非常に率直ですが、複雑なエラーメッセージを吐いたり、予期せぬ振る舞いが見つかったりします。

単純な例として、トレイトオブジェクトの実行時の表現から見て行きましょう。 `std::raw` モジュールは複雑なビルドインの型と同じレイアウトの構造体を格納しており、 [トレイトオブジェクトも含まれています](https://doc.rust-lang.org/std/raw/) 。

```rust
# mod foo {
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}
# }
```

つまり、 `&Foo` のようなトレイトオブジェクトは「data」ポインタと「vtable」ポインタから成るわけです。

dataポインタはトレイトオブジェクトが保存している（何らかの不明な型 `T` の）データを指しており、vtableポインタは `T` への `Foo` の実装に対応するvtable（「virtual method table」）を指しています。

vtableは本質的には関数ポインタの構造体で、実装内における各メソッドの具体的な機械語の命令列を指しています。 `trait_object.method()` のようなメソッド呼び出しを行うとvtableの中から適切なポインタを取り出し動的に呼び出しを行います。例えば、

```rust
struct FooVtable {
    destructor: fn(*mut ()),
    size: usize,
    align: usize,
    method: fn(*const ()) -> String,
}

// u8:

fn call_method_on_u8(x: *const ()) -> String {
    // コンパイラは `x` がu8を指しているときにのみこの関数が呼ばれることを保障します
    let byte: &u8 = unsafe { &*(x as *const u8) };

    byte.method()
}

static Foo_for_u8_vtable: FooVtable = FooVtable {
    destructor: /* コンパイラマジック */,
    size: 1,
    align: 1,

    // 関数ポインタへキャスト
    method: call_method_on_u8 as fn(*const ()) -> String,
};


// String:

fn call_method_on_String(x: *const ()) -> String {
# //     // the compiler guarantees that this function is only called
# //     // with `x` pointing to a String
    // コンパイラは `x` がStringを指しているときにのみこの関数が呼ばれることを保障します
    let string: &String = unsafe { &*(x as *const String) };

    string.method()
}

static Foo_for_String_vtable: FooVtable = FooVtable {
    destructor: /* コンパイラマジック */,
    // この値は64bitコンピュータ向けのものです、32bitコンピュータではこの半分にします
    size: 24,
    align: 8,

    method: call_method_on_String as fn(*const ()) -> String,
};
```

各vtableの `destructor` フィールドはvtableが対応する型のリソースを片付ける関数を指しています。 `u8` のvtableは単純な型なので何もしませんが、 `String` のvtableはメモリを解放します。 このフィールドは `Box<Foo>` のような自作トレイトオブジェクトのために必要であり、 `Box` によるアロケートは勿論のことスコープ外に出た際に内部の型のリソースを片付けるのにも必要です。 `size` 及び `align` フィールドは消去された型のサイズとアライメント要件を保存しています。 これらの情報はデストラクタにも組み込まれているため現時点では基本的に使われていませんが、将来、トレイトオブジェクトがより柔軟になることで使われるようになるでしょう。

例えば `Foo` を実装する値を幾つか得たとします。 `Foo` トレイトオブジェクトを作る、あるいは使う時のコードを明示的に書いたものは少しだけ似ているでしょう。 （型の違いを無視すればですが、どのみちただのポインタになります）

```rust
let a: String = "foo".to_string();
let x: u8 = 1;

// let b: &dyn Foo = &a;
let b = TraitObject {
    // データを保存
    data: &a,
    // メソッドを保存
    vtable: &Foo_for_String_vtable
};

// let y: &dyn Foo = &x;
let y = TraitObject {
    // データを保存
    data: &x,
    // メソッドを保存
    vtable: &Foo_for_u8_vtable
};

// b.method();
(b.vtable.method)(b.data);

// y.method();
(y.vtable.method)(y.data);
```
