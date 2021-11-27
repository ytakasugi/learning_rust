td::fs

---

#### std::fs::File

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
