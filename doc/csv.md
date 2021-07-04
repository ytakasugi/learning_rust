### [Crate csv](https://docs.rs/csv/1.1.6/csv/)

csv クレートは、Serde をサポートした、高速で柔軟な CSV リーダーとライターを提供します。

Rustが初めての方は[チュートリアル](https://docs.rs/csv/1.1.6/csv/tutorial/index.html)から始めるのが良いでしょう。

[cookbook](https://docs.rs/csv/1.1.6/csv/cookbook/index.html)には、CSVの読み書きを行う様々な完全なRustプログラムが掲載されています。

---

- Brief overview

  Rustを初めて使う方は、[チュートリアル](https://docs.rs/csv/1.1.6/csv/tutorial/index.html)を参考にしてみてください。

  このクレートの主な型は[`Reader`](https://docs.rs/csv/1.1.6/csv/struct.Reader.html)と[`Writer`](https://docs.rs/csv/1.1.6/csv/struct.Writer.html)で、それぞれCSVデータを読み書きします。これに対応して、カスタムフィールドやレコードデリミタを持つCSVデータをサポートするには、CSVデータを読み書きするかどうかに応じて、[`ReaderBuilder`](https://docs.rs/csv/1.1.6/csv/struct.ReaderBuilder.html)または[`WriterBuilder`](https://docs.rs/csv/1.1.6/csv/struct.WriterBuilder.html)を使用する必要があります。

  `Serde`を使用している場合を除き、標準的なCSVレコードタイプは[`StringRecord`](https://docs.rs/csv/1.1.6/csv/struct.StringRecord.html)と[`ByteRecord`](https://docs.rs/csv/1.1.6/csv/struct.ByteRecord.html)です。`StringRecord`は、データが有効なUTF-8であることがわかっている場合に使用します。UTF-8ではないかもしれないデータの場合は、`ByteRecord`が適しています。

  最後に、エラーのセットは、[`Error`](https://docs.rs/csv/1.1.6/csv/struct.Error.html)タイプによって記述されます。

  このクレート内の残りのタイプは、主に、より詳細なエラー、位置情報、構成ノブ、またはイテレータタイプに対応します。

---

- Setup

  `Cargo.toml`に以下を追加します。

  ```
  [dependencies]
  csv = "1.1"
  ```

  カスタム構造体に`Serde`のカスタム導出機能を使用したい場合は、`argo.toml`の`[dependencies]`セクションにこれを追加します。

---

- Example

  この例では、標準入力からCSVデータを読み込み、各レコードを標準出力に出力する方法を示します。

  ク[cookbook](https://docs.rs/csv/1.1.6/csv/cookbook/index.html)にはさらに多くの例があります。

  ```rust
  use std::error::Error;
  use std::io;
  use std::process;
  
  fn example() -> Result<(), Box<dyn Error>> {
      // Build the CSV reader and iterate over each record.
      let mut rdr = csv::Reader::from_reader(io::stdin());
      for result in rdr.records() {
          // The iterator yields Result<StringRecord, Error>, so we check the
          // error here.
          let record = result?;
          println!("{:?}", record);
      }
      Ok(())
  }
  
  fn main() {
      if let Err(err) = example() {
          println!("error running example: {}", err);
          process::exit(1);
      }
  }
  ```

  上記の例は次のように実行できます。

  ```
  $ git clone git://github.com/BurntSushi/rust-csv
  $ cd rust-csv
  $ cargo run --example cookbook-read-basic < examples/data/smallpop.csv
  ```

---

- Example with Serde

  この例では、標準入力からCSVデータを読み込んで、独自のカスタム構造体を作成します。デフォルトでは、構造体のメンバ名とCSVデータのヘッダレコードの値が一致します。

  ```rust
  use std::error::Error;
  use std::io;
  use std::process;
  
  use serde::Deserialize;
  
  #[derive(Debug, Deserialize)]
  struct Record {
      city: String,
      region: String,
      country: String,
      population: Option<u64>,
  }
  
  fn example() -> Result<(), Box<dyn Error>> {
      let mut rdr = csv::Reader::from_reader(io::stdin());
      for result in rdr.deserialize() {
          // Notice that we need to provide a type hint for automatic
          // deserialization.
          let record: Record = result?;
          println!("{:?}", record);
      }
      Ok(())
  }
  
  fn main() {
      if let Err(err) = example() {
          println!("error running example: {}", err);
          process::exit(1);
      }
  }
  ```

  上記の例は次のように実行できます。

  ```
  $ git clone git://github.com/BurntSushi/rust-csv
  $ cd rust-csv
  $ cargo run --example cookbook-read-serde < examples/data/smallpop.csv
  ```

  

