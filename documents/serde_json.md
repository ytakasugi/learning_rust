### Create [`serde_json`](https://docs.serde.rs/serde_json/index.html)

JSONは、人間が読めるテキストを使って、キーと値のペアからなるデータオブジェクトを送信する、ユビキタスなオープンスタンダードのフォーマットです。

```json
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}
```

JSONデータをRustで扱う際には、3つの一般的な方法があります。

- テキストデータとして。HTTPエンドポイントで受信したり、ファイルから読み込んだり、リモートサーバーに送信する準備をしたりする、処理されていないJSONデータの文字列です。
- 型付けされていない、または緩く型付けされた表現として。JSONデータを渡す前に、そのデータが有効であるかどうかを確認したいが、そのデータに含まれるものの構造は知らない、という場合もあるでしょう。また、特定の場所にキーを挿入するなど、非常に基本的な操作を行いたい場合もあります。
- Rustの強力に型付けされたデータ構造として。データのすべてまたは大部分が特定の構造に準拠していることを期待し、JSON の緩い性質に惑わされることなく実際の作業を行いたい場合に使用します。

Serde JSON は、これらの表現間でデータを変換する、効率的で柔軟かつ安全な方法を提供します。

---

### Operating on untyped JSON values

任意の有効なJSONデータは、以下の再帰的なenum表現で操作できます。このデータ構造は[`serde_json::Value`](https://docs.serde.rs/serde_json/value/enum.Value.html)です。

```rust
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

JSONデータの文字列は、[`serde_json::from_str`](https://docs.serde.rs/serde_json/de/fn.from_str.html)関数で`serde_json::Value`に解析することができます。また、バイトスライス`&u8`からの解析を行う[`from_slice`](https://docs.serde.rs/serde_json/de/fn.from_slice.html)や、FileやTCPストリームのような任意の`io::Read`からの解析を行う [`from_reader`](https://docs.serde.rs/serde_json/de/fn.from_reader.html)もあります。

```rust
use serde_json::{Result, Value};

fn untyped_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}
```

`v["name"]`のような角括弧によるインデックス付けの結果は、そのインデックスのデータの借用であるため、型は`&Value`となります。JSONマップは文字列のキーでインデックスを付けることができ、JSON配列は整数のキーでインデックスを付けることができます。データの型がインデックスされている型に対して正しくない場合、マップにインデックスされているキーが含まれていない場合、またはベクターのインデックスが範囲外である場合、返される要素は `Value::Null`です。

`Value`が出力されると、それはJSON文字列として出力されます。つまり、上のコードでは、`Please call "John Doe" at the number "44 1234567 "`のように出力されます。引用符が表示されているのは、v["name"]がJSON文字列を含む&Valueであり、そのJSON表現が` "John Doe "`であるためです。引用符のないプレーンな文字列として印刷するには、JSON文字列を`as_str()`でRust文字列に変換するか、次のセクションで説明するように`Value`の使用を避ける必要があります。

`Value`表現は、非常に基本的な作業には十分ですが、それ以上の作業には退屈なものになります。例えば、入力データに認識されないフィールドがあることを検出しようとすると、エラー処理を正しく実装するためには冗長になります。例えば、`v["name"]`を`v["nmae"]`とタイプミスしてしまった場合、コンパイラは助けてくれません。

---

### Parsing JSON as strongly typed data structures

Serdeは、JSONデータをRustのデータ構造にほぼ自動的にマッピングする強力な方法を提供します。

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}
```

これは先ほどと同じ`serde_json::from_str`関数ですが、今回は戻り値を`Person`型の変数に代入しています。そうすることで Serdeは入力データを自動的に`Person`として解釈し、レイアウトが`Person`として期待されるものと一致しない場合には、有益なエラーメッセージを生成します。

Serdeの`Deserialize`トレイトを実装している型であれば、この方法でデシリアライズすることができます。これには、`Vec<T>`や `HashMap<K, V>`のような Rust標準ライブラリの組み込み型や、`#[derive(Deserialize)]`でアノテーションされた構造体やenumも含まれます。

`Person`型の`p`ができたら、他のRustコードと同じように、IDEとRustコンパイラが正しく使えるようにしてくれます。IDEはフィールド名をオートコンプリートしてタイプミスを防ぎますが、これは`serde_json::Value`表現では不可能でした。また、Rustコンパイラは、`p.phones[0]`と書いたときに、`p.phones`が`Vec<String>`であることが保証されているので、それにインデックスを付けることが意味を持ち、Stringを生成することをチェックしてくれます。

---

### Constructing JSON values

Serde JSONは、非常に自然なJSON構文で`serde_json::Value`オブジェクトを構築する`json!`マクロを提供しています。

```rust
use serde_json::json;

fn main() {
    // The type of `john` is `serde_json::Value`
    let john = json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    println!("first phone number: {}", john["phones"][0]);

    // Convert to a string of JSON and print it out
    println!("{}", john.to_string());
}
```

`Value::to_string()`関数は、`serde_json::Value`をJSONテキストのStringに変換します。

`json! `マクロの優れた点は、JSONの値を構築する際に、変数や式を直接補間できることです。Serde`はコンパイル時に、補間している値がJSONとして表現できるかどうかをチェックします。

```rust
let full_name = "John Doe";
let age_last_year = 42;

// The type of `john` is `serde_json::Value`
let john = json!({
    "name": full_name,
    "age": age_last_year + 1,
    "phones": [
        format!("+44 {}", random_phone())
    ]
});
```

これは驚くほど便利ですが、以前の`Value`の時のように、間違ってもIDEやRustコンパイラーが助けてくれないという問題があります。Serde JSON は、strong-typedのデータ構造を JSON テキストにシリアライズするより良い方法を提供します。

---

### Creating JSON by serializing data structures

データ構造は、[`serde_json::to_string`](https://docs.serde.rs/serde_json/ser/fn.to_string.html)でJSON文字列に変換できます。また、`Vec<u8>`にシリアライズする[`serde_json::to_vec`](https://docs.serde.rs/serde_json/ser/fn.to_vec.html)や、FileやTCPストリームなどの任意の`io::Write`にシリアライズする[`serde_json::to_writer`](https://docs.serde.rs/serde_json/ser/fn.to_writer.html)もあります。

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}
```

SerdeのSerializeトレイトを実装している型であれば、この方法でシリアル化することができます。これには、`Vec<T>`や`HashMap<K, V>`のようなRust標準ライブラリの組み込み型や、`#[derive(Serialize)]`でアノテーションされた構造体やenumも含まれます。

---

### No-std support

メモリアロケータがある限り、Rust標準ライブラリの残りの部分を使わずにserde_jsonを使うことができます。これはRust 1.36+でサポートされています。デフォルトの "std "機能を無効にして、"alloc "機能を有効にします。

```toml
[dependencies]
serde_json = { version = "1.0", default-features = false, features = ["alloc"] }
```

メモリアロケータを使用しないSerdeでのJSONサポートについては、 [`serde-json-core`](https://japaric.github.io/serde-json-core/serde_json_core/)クレートをご覧ください。

