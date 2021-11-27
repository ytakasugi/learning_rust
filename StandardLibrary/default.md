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

