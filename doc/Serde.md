### Overview

- [Serde](https://docs.serde.rs/serde/index.html)

  `Serde`は、Rust データ構造を効率的かつ汎用的に`Serialize`/`Deserialize`するためのフレームワークです。

  `Serde`のエコシステムは、自分自身をシリアル化/デシリアル化する方法を知っているデータ構造と、他のものをシリアル化/デシリアル化する方法を知っているデータフォーマットで構成されています。`Serde`は、この 2 つのグループが相互に作用するためのレイヤーを提供し、サポートされているデータ構造を、サポートされているデータフォーマットを使用してシリアル化およびシリアル化解除できるようにします。

- Design

  他の多くの言語では、データのシリアライズをランタイムリフレクションに依存していますが、`Serde`は代わりに Rust の強力な trait システムに基づいて構築されています。自分自身をシリアライズ/デシリアライズする方法を知っているデータ構造は、`Serde`の`Serialize`および`Deserialize`トレイトを実装している (または`Serde`の`derive `属性を使用してコンパイル時に実装を自動生成している) データ構造です。これにより、リフレクションやランタイム型情報のオーバーヘッドを回避できます。実際、多くの状況において、データ構造とデータフォーマットの間の相互作用は、Rustコンパイラによって完全に最適化され、`Serde`シリアライズは、データ構造とデータフォーマットの特定の選択に対して、手書きのシリアライザと同じ速度で実行されます。

- Data formats

  以下は、コミュニティによって`Serde`に実装されたデータフォーマットの一部です。

  - [JSON](https://github.com/serde-rs/json)：多くのHTTP APIで使用されているユビキタスなJavaScript Object Notation。
  - [Bincode](https://github.com/servo/bincode)：Servoレンダリングエンジン内のIPCに使用されるコンパクトなバイナリフォーマット。
  - [CBOR](https://github.com/pyfisch/cbor)：バージョンネゴシエーションを必要としない小さなメッセージサイズのために設計された簡潔なバイナリオブジェクト表現。
  - [YAML](https://github.com/dtolnay/serde-yaml)マークアップ言語ではないが人に優しい設定言語と自称しています。
  - [MessagePack](https://github.com/3Hren/msgpack-rust)：コンパクトなJSONに似た効率的なバイナリフォーマット。
  - [TOML](https://github.com/alexcrichton/toml-rs)：Cargoで使用されている最小限の設定フォーマットです。
  - [Pickle](https://github.com/birkenfeld/serde-pickle)：Pythonの世界では一般的なフォーマットです。
  - [RON](https://github.com/ron-rs/ron)：Rusty Object Notation（ラスティ・オブジェクト・ノーテーション）の略。
  - [RON](https://github.com/ron-rs/ron)：MongoDBで使用されているデータストレージとネットワーク転送のフォーマット。
  - [Avro](https://github.com/flavray/avro-rs)：Apache Hadoopで使用されているバイナリフォーマットで、スキーマ定義をサポートしています。
  - [JSON5](https://github.com/callum-oakley/json5-rs)：ES5の一部を含むJSONのスーパーセットです。
  - [Postcard](https://github.com/jamesmunns/postcard)：`no_std`および組み込みシステムに適したコンパクトなバイナリフォーマット。
  -  [URL](https://docs.rs/serde_qs)：`x-www-form-urlencode`形式のURLクエリ文字列です。
  - [Envy](https://github.com/softprops/envy)：環境変数を Rust の構造体にデシリアライズする方法です。(デシリアライズのみ)
  - [Envy Store](https://github.com/softprops/envy-store)：[AWS Parameter Store](https://docs.aws.amazon.com/systems-manager/latest/userguide/systems-manager-paramstore.html) パラメータを Rust 構造体にデシリアライズする方法です。(デシリアライズのみ)
  - [S-expressions](https://github.com/rotty/lexpr-rs)： Lisp言語ファミリーで使用されているコードとデータのテキスト表現です。
  - [D-Bus](https://docs.rs/zvariant)：バイナリワイヤフォーマット。
  - [FlexBuffers](https://github.com/google/flatbuffers/tree/master/rust/flexbuffers)：GoogleのFlatBuffersのゼロコピーシリアル化フォーマットのスキームレスの従兄弟です。
  - [Bencode](https://github.com/P3KI/bendy)：BitTorrent プロトコルで使用されるシンプルなバイナリ形式です。
  - [DynamoDB Items](https://docs.rs/serde_dynamo)：[`rusoto_dynamodb`](https://docs.rs/rusoto_dynamodb)がDynamoDBとの間でデータを転送する際に使用するフォーマット。
  - [Hjson](https://github.com/Canop/deser-hjson)：人間による読み取りと編集を目的としたJSONの拡張構文です。(デシリアライズのみ)

- Data structures

  `Serde`はすぐに、Rust の一般的なデータ型を上記のいずれかの形式でシリアライズ/デシリアライズすることができます。例えば、`String`, `&str`, `usize`, `Vec<T>`, `HashMap<K,V>`などがサポートされています。さらに、`Serde`は構造体のシリアライズ実装を自分のプログラムで生成するための`derive`マクロを提供しています。deriveマクロの使い方は次のようになります。

  ```rust
  use serde::{Serialize, Deserialize};
  
  #[derive(Serialize, Deserialize, Debug)]
  struct Point {
      x: i32,
      y: i32,
  }
  
  fn main() {
      let point = Point { x: 1, y: 2 };
  
      // Convert the Point to a JSON string.
      let serialized = serde_json::to_string(&point).unwrap();
  
      // Prints serialized = {"x":1,"y":2}
      println!("serialized = {}", serialized);
  
      // Convert the JSON string back to a Point.
      let deserialized: Point = serde_json::from_str(&serialized).unwrap();
  
      // Prints deserialized = Point { x: 1, y: 2 }
      println!("deserialized = {:?}", deserialized);
  }
  ```


---

### Serde data model

- Serde data model

  `Serde`のデータモデルは、データ構造とデータフォーマットが相互に作用するためのAPIです。これは`Serde`の型システムと考えることができます。

  コード上では、`Serde`データモデルの半分のシリアライズは`Serializer`トレイトで定義され、半分のデシリアライズは `Deserializer`トレイトで定義されます。これらは、すべてのRustデータ構造を29の可能な型の1つにマッピングする方法です。`Serializer`トレイトの各メソッドは、データモデルの型の1つに対応しています。

  データ構造を何らかのフォーマットにシリアライズする場合、データ構造に対するSerializeの実装は、`Serializer`のメソッドを1つだけ呼び出してデータ構造を`Serde`データモデルにマッピングする責任があり、データフォーマットに対する`Serializer`の実装は、`Serde`データモデルを意図した出力表現にマッピングする責任があります。

  データ構造を何らかのフォーマットからデシリアライズする場合、データ構造用のデシリアライズ実装は、データモデルのさまざまなタイプを受け取ることができるビジター実装をデシリアライザに渡すことによって、データ構造を`Serde`データモデルにマッピングする責任があり、一方、データフォーマット用のデシリアライザ実装は、ビジターメソッドのうちの正確に1つを呼び出すことによって、入力データを`Serde`データモデルにマッピングする責任があります。

- Types

  `Serde`データモデルは、Rustの型システムを簡略化したものです。以下の29種類の型で構成されています。

  - 14 primitive types
    - `bool`
    - `i8`, `i16`, `i32`, `i64`, `i128`
    - `u8`, `u16`, `u32`, `u64`, `u128`
    - `f32`, `f64`
    - `char`
  - string
    - UTF-8のバイトで、長さがあり、NULLターミネーターはありません。0バイトを含んでも構いません。
    - シリアル化の際には、すべての文字列が同じように扱われます。デシリアライズ時には、文字列には一時的なもの、所有されているもの、借用されているものの 3 種類があります。この区別は、[Understanding deserializer lifetimes](https://serde.rs/lifetimes.html)で説明されており、`Serde`が効率的なゼロコピーデシリアライゼーションを可能にした重要な方法です。.
  - byte array - [u8]
    - 文字列と同様に、デシリアライズ時にバイト配列は一過性のものになったり、所有されたり、借りられたりします。
  - option
    - 何もないか、何かの価値があるかのどちらかです。
  - unit
    - Rustでの`()`の型。データを含まない無名の値を表します。
  - unit_struct
    - 例えば、`struct Unit`や`PhantomData<T>`などです。データを含まない名前付きの値を表します。
  - unit_variant
    - 例えば、 `E::A` and `E::B` in `enum E { A, B }`.
  - newtype_struct
    - 例えば、`struct Millimeters(u8)`.
  - newtype_variant
    - 例えば、 `E::N` in `enum E { N(u8) }`.
  - seq
    - `Vec<T>`や`HashSet<T>`のような、サイズの異なる異種の値のシーケンスです。シリアル化する際には、すべてのデータを繰り返し処理する前に長さを知ることができますが、そうでない場合もあります。デシリアライズ時には、シリアル化されたデータを見て長さを決定します。`vec![Value::Bool(true), Value::Char('c')]`のような同種の Rust コレクションは、異種のSerde seqとしてシリアライズされる可能性があることに注意してください（この場合、Serde boolの後にSerde charが続きます）。
  - tuple
    - 例えば、`(u8,)`や`(String, u64, Vec<T>)`や`[u64; 10]`のように、シリアル化されたデータを見なくても、デシリアライズ時に長さが分かるような、静的なサイズの異種の値のシーケンスです。
  - tuple_struct
    - 名前付きタプル、例えば `struct Rgb(u8, u8, u8)`.
  - tuple_variant
    - 例えば、 `E::T` in `enum E { T(u8, u8) }`.
  - map
    - `BTreeMap<K, V>`のような、可変サイズの異種キー・バリューペア。シリアル化する際には、すべてのエントリを繰り返し処理する前に、長さを知ることができる場合とできない場合があります。デシリアライズする際には、シリアル化されたデータを見て長さを決定します。
  - struct
    - `struct S { r: u8, g: u8, b: u8 }`のように、キーがコンパイル時の定数文字列であり、シリアル化されたデータを見なくても、デシリアライズ時に知ることができる、静的サイズの異種混合のキーと値のペア。
  - struct_variant
    - 例えば、 `E::S` in `enum E { S { r: u8, g: u8, b: u8 } }`.

- Mapping into the data model

  ほとんどのRustの型の場合，`Serde`のデータモデルへのマッピングは簡単です。例えば、Rustのbool型は`Serde`のbool型に対応します。`Rust`の タプル構造体 `Rgb(u8, u8, u8)`は`Serde`のタプル構造体型に対応します。

  しかし、これらのマッピングが素直でなければならない基本的な理由はありません。`Serialize`と`Deserialize`のトレイトは、ユースケースに適したRustの型と`Serde`のデータモデルの間の任意のマッピングを実行できます。

  例として、Rust の`std::ffi::OsString`型を考えてみましょう。この型は、プラットフォームネイティブな文字列を表します。Unix システムでは、任意の非ゼロのバイトであり、Windowsシステムでは、任意の非ゼロの 16 ビット値です。`OsString`を`Serde`データモデルに以下の型のひとつとしてマッピングするのは、自然なことのように思えるかもしれません。

  - `Serde`の文字列として。残念ながら、`OsString`はUTF-8で表現できることが保証されていないため、シリアライズは脆くなり、`Serde`の文字列は0バイトを含むことができるため、デシリアライズも脆くなります。

  - `Serde`のバイト配列として。これで、文字列を使った場合の問題は両方とも解決しましたが、今度は、Unixで`OsString`をシリアライズして、Windowsでデシリアライズすると、間違った文字列になってしまいます。

  代わりに、`OsString`用の`Serialize`および`Deserialize`の実装は、`OsString`を `Serde enum`として扱うことで`Serde`データモデルにマッピングします。実際には、`OsString`が以下の型として定義されているかのように動作しますが、これは個々のプラットフォームでの定義とは一致しません。

  ```rust
  enum OsString {
      Unix(Vec<u8>),
      Windows(Vec<u16>),
      // and other platforms
  }
  ```

  `Serde`のデータモデルへのマッピングの柔軟性は非常に高く、強力です。`Serialize`と`Deserialize`を実装する際には、直感的なマッピングが最良の選択ではないかもしれない、型の広範な文脈を意識してください。

---

### Using derive

- Using derive

  `Serde`は、自分のクレートで定義されたデータ構造の`Serialize`および `Deserialize`トレイトの実装を生成する derive マクロを提供し、`Serde`のすべてのデータフォーマットで便利に表現できるようにしています。

  コードが` #[derive(Serialize, Deserialize)]`を使用している場合のみ、この設定が必要です。

  この機能はRustの`#[derive]`メカニズムに基づいており、ビルトインの`Clone`、`Copy`、`Debug`、その他のトレイトの実装を自動的に導き出すのに使うようなものです。この機能は、精巧な汎用型や trait の境界を持つものを含め、ほとんどの構造体や `num`の実装を生成することができます。まれに、特に複雑な型の場合は、手作業でトレイトを実装する必要があるかもしれません。

  これらの導出には、Rustコンパイラのバージョン1.31以降が必要です。

  - `Cargo.toml`に`serde = { version = "1.0", features = ["derive"] }`を依存関係として追加します。
  - 他のすべての`Serde`ベースの依存関係（例えば`serde_json`）がserde 1.0と互換性のあるバージョンであることを確認してください。
  - シリアライズしたい構造体や列挙体には、同じモジュール内で`use serde::Serialize;`として`derive`マクロをインポートし、構造体や列挙体に `#[derive(Serialize)]`と記述します。
  - 同様に，`serde::Deserialize;`をインポートして，デシリアライズしたい構造体や`enum`に`#[derive(Deserialize)]`と記述します。

  ここでは、Cargo.tomlを紹介します。

  ```toml
  [package]
  name = "my-crate"
  version = "0.1.0"
  authors = ["Me <user@rust-lang.org>"]
  
  [dependencies]
  serde = { version = "1.0", features = ["derive"] }
  
  # serde_json is just for the example, not required in general
  serde_json = "1.0"
  ```

  ```rust
  use serde::{Serialize, Deserialize};
  
  #[derive(Serialize, Deserialize, Debug)]
  struct Point {
      x: i32,
      y: i32,
  }
  
  fn main() {
      let point = Point { x: 1, y: 2 };
  
      let serialized = serde_json::to_string(&point).unwrap();
      println!("serialized = {}", serialized);
  
      let deserialized: Point = serde_json::from_str(&serialized).unwrap();
      println!("deserialized = {:?}", deserialized);
  }
  ```

  以下はその出力結果です。

  ```
  $ cargo run
  serialized = {"x":1,"y":2}
  deserialized = Point { x: 1, y: 2 }
  ```

- Troubleshooting

  時々、コンパイル時のエラーが表示されることがあります。

  ```
  the trait `serde::ser::Serialize` is not implemented for `...`
  ```

  構造体や列挙体に明らかに`#[derive(Serialize)]`と書かれているにもかかわらずです。

  これはほとんどの場合、互換性のないバージョンの`Serde`に依存するライブラリを使用していることを意味します。`Cargo.toml`ではserde 1.0に依存していても、serde 0.9に依存している他のライブラリを使っている場合があります。つまり、serde 1.0のSerializeトレイトが実装されていても、ライブラリはserde 0.9のSerializeトレイトの実装を期待しているのです。Rustコンパイラの視点では、これらは全く異なるトレイトです。

  修正方法は、Serdeのバージョンが一致するまで、必要に応じてライブラリをアップグレードまたはダウングレードすることです。`cargo tree -d`コマンドは、重複した依存関係が引き込まれているすべての場所を見つけるのに役立ちます。

---

### Attributes

属性は、Serde's derive が生成する Serialize および Deserialize の実装をカスタマイズするために使用されます。属性を使用するには、Rust コンパイラのバージョン 1.15 以降が必要です。

属性には 3 つのカテゴリがあります。

- コンテナ属性 - 構造体や列挙体の宣言に適用されます。
- Variant属性 - 列挙体のバリアントに適用されます。
- フィールド属性 - 構造体または列挙体のバリアントの1つのフィールドに適用されます。

```rust
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // <-- this is a container attribute
struct S {
    #[serde(default)]  // <-- this is a field attribute
    f: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename = "e")]  // <-- this is also a container attribute
enum E {
    #[serde(rename = "a")]  // <-- this is a variant attribute
    A(String),
}
```

1つの構造体、列挙体、列挙子、またはフィールドが複数の属性を持つことがあることに注意してください。

- Container attributes

  - ##### `#[serde(rename = "name")]`

    この構造体や列挙体を、Rust名ではなく指定された名前でシリアル化およびデシリアル化します。

    シリアライズとデシリアライズにそれぞれ独立した名前を指定できます。

    - `#[serde(rename(serialize = "ser_name"))]`
    - `#[serde(rename(deserialize = "de_name"))]`
    - `#[serde(rename(serialize = "ser_name", deserialize = "de_name"))]`

  - ##### `#[serde(rename_all = "...")]`

    すべてのフィールド（これが構造体の場合）またはバリアント（これが列挙型の場合）の名前を、指定された大文字小文字の区別に従って変更します。指定できる値は `"lowercase"`, `"UPPERCASE"`, `"PascalCase"`, `"camelCase"`, `"snake_case"`, `"SCREAMING_SNAKE_CASE"`, `"kebab-case"`, `"SCREAMING-KEBAB-CASE"`.

    シリアル化とデシリアル化の独立したケースを指定できます。

    - `#[serde(rename_all(serialize = "..."))]`
    - `#[serde(rename_all(deserialize = "..."))]`
    - `#[serde(rename_all(serialize = "...", deserialize = "..."))]`

  - ##### `#[serde(deny_unknown_fields)]`

    未知のフィールドに遭遇した場合、デシリアライズ中に常にエラーを発生させます。この属性がない場合、JSONなどの自己記述的なフォーマットでは、デフォルトで未知のフィールドは無視されます。

    注：この属性は、外側の構造体やフラット化されたフィールドではなく、[`flatten`](https://serde.rs/field-attrs.html#flatten)との組み合わせではサポートされません。

  - ##### `#[serde(tag = "type")]`

    このenumの内部的にタグ付けされたenum表記法を、指定されたタグで使用します。この表現の詳細については、[enum representations](https://serde.rs/enum-representations.html)を参照してください。

  - ##### `#[serde(tag = "t", content = "c")]`

    この enumの隣接タグ付き enum表記法を、タグとコンテンツに指定されたフィールド名で使用します。この表現の詳細については、[enum representations](https://serde.rs/enum-representations.html)を参照してください。

  - ##### `#[serde(untagged)]`

    このenumにはタグなしのenum表記法を使用します。この表現の詳細については、[enum representations](https://serde.rs/enum-representations.html)を参照してください。

  - ##### `#[serde(bound = "T: MyTrait")]`

    `Serialize`および`Deserialize`実装のためのwhere句。これは、`Serde`によって推測されたトレイトの境界を置き換えるものです。

    シリアル化とデシリアル化にそれぞれ独立した境界を指定することができます。

    - `#[serde(bound(serialize = "T: MySerTrait"))]`
    - `#[serde(bound(deserialize = "T: MyDeTrait"))]`
    - `#[serde(bound(serialize = "T: MySerTrait", deserialize = "T: MyDeTrait"))]`

  - ##### `#[serde(default)]`

    デシリアライズ時には、不足しているフィールドは、構造体のDefaultの実装から埋められる必要があります。構造体でのみ使用できます。

  - ##### `#[serde(default = "path")]`

    デシリアライズする際には、不足しているフィールドは、与えられた関数やメソッドが返すオブジェクトから埋めなければなりません。例えば、`default = "my_default"`は`my_default()`を、`default = "SomeTrait::some_default"`は`SomeTrait::some_default()`を呼び出すことができます。構造体でのみ使用できます。

  - ##### `#[serde(remote = "...")]`

    これは、[remote types](https://serde.rs/remote-derive.html)の`Serialize`と`Deserialize`を派生させるために使用されます。

  - ##### `#[serde(transparent)]`

    ニュータイプ構造体または1つのフィールドを持つ波括弧付きの構造体を、その1つのフィールドがそれ自体でシリアル化およびデシリアル化された場合とまったく同じようにシリアル化およびデシリアル化します。これは` #[repr(transparent)]`と似ています。

  - ##### `#[serde(from = "FromType")]`

    この型を`FromType`にデシリアライズし、変換します。この型は `From<FromType>`を実装しなければならず、`FromType`は`Deserialize`を実装しなければなりません。

  - ##### `#[serde(try_from = "FromType")]`

    この型を`FromType`にデシリアライズしてから、誤りなく変換します。このタイプは`Display`を実装したエラータイプで`TryFrom<FromType>`を実装しなければならず、`FromType`は`Deserialize`を実装しなければなりません。

  - ##### `#[serde(into = "IntoType")]`

    このタイプを指定された`IntoType`に変換してシリアル化します。このタイプは`Clone`と`Into<IntoType>`を実装しなければならず、`IntoType`は`Serialize`を実装しなければなりません。

  - ##### `#[serde(crate = "...")]`

    生成されたコードから Serde API を参照する際に使用する serde クレート インスタンスへのパスを指定します。これは通常、別のクレートのパブリックマクロから再エクスポートされた Serde 派生物を呼び出す場合にのみ適用されます。


---

- Variant attributes

  

  

  

