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
