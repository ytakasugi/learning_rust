### std::io

- Description

  I/Oのコア機能のためのトレイト、ヘルパー、型定義です。

  `std::io`モジュールには、入出力を行う際に必要となる一般的なものが数多く含まれています。このモジュールの最も中心的な部分は、`Read`と`Write`のトレイトであり、入出力を読み書きするための最も一般的なインタフェースを提供しています。

---

#### std::io::BufReader<R>

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

#### std::io::Result

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

#### std::io::stdin

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

#### std::io::stdout

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

#### std::io::Stdout::flush

- Description

  この出力ストリームをフラッシュして、一時的にバッファリングされたコンテンツがすべて目的地に到達するようにします。



---

#### std::io::Read::read

- Description

  このソースからいくつかのバイトを指定されたバッファに引き込み、何バイト読まれたかを返します。

  この関数は、データの待ち受けをブロックするかどうかについては何の保証もしませんが、オブジェクトが読み込みのためにブロックする必要があり、ブロックできない場合は、通常は`Err`返り値を介してその旨を通知します。

  このメソッドの戻り値が Ok(n) である場合、`0 <= n <= buf.len()`であることが保証されなければなりません。ゼロでない`n`の値は、バッファ`buf`がこのソースからの`n`バイトのデータで埋め尽くされたことを示します。`n`が 0 の場合は、2 つのシナリオのうちの 1 つを示します。

---

### std::io::Read::read::read_exact

- Description

  `buf`を埋めるのに必要な正確なバイト数を読み込む。

  この関数は、指定されたバッフ`buf`を完全に満たすのに必要な数のバイトを読み込む。

  この関数が呼ばれたとき、`buf`の内容については何も保証されていないので、実装は`buf`の内容のどのプロパティも真であることに頼ることはできない。実装では、`buf`の内容を読むのではなく、`buf`にデータを書き込むだけにしておくことを推奨する。この問題については、[`read`](https://doc.rust-lang.org/stable/std/io/trait.Read.html#tymethod.read)のドキュメントに詳細な説明があります。

- Errors

  この関数が[`ErrorKind::Interrupted`](https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#variant.Interrupted)の種類のエラーに遭遇した場合、そのエラーは無視され、操作は続行されます。

  この関数がバッファを完全に満たす前に "end of file"に遭遇した場合は、[`ErrorKind::UnexpectedEof`](https://doc.rust-lang.org/stable/std/io/enum.ErrorKind.html#variant.UnexpectedEof)の種類のエラーを返す。この場合、`buf`の内容は特定されない。

  その他の読み込みエラーが発生した場合、本関数は直ちに戻る。この場合、`buf` の内容は特定されない。

  本関数がエラーを返した場合、何バイト読み込んだかは不明であるが、バッファを完全に満たすのに必要な量以上を読み込むことはない。

- Exmaple

  [`File`](https://doc.rust-lang.org/stable/std/fs/struct.File.html)s implement `Read`:

```rust
  use std::io;
  use std::io::prelude::*;
  use std::fs::File;
  
  fn main() -> io::Result<()> {
      let mut f = File::open("foo.txt")?;
      let mut buffer = [0; 10];
  
      // read exactly 10 bytes
      f.read_exact(&mut buffer)?;
      Ok(())
  }
```




---

#### std::io::Read::read_to_string()

  - Description

    End of Fileまですべてのバイトを読み込みバッファに追加する。

    成功した場合、この関数は読み込んでバッファに追加したバイト数を返却する。

    このストリーム内のデータが有効なUTF-8でない場合、エラーを返却し、バッファに追加されない。

    そのほかの、エラーセマンティクスは[read_to_end](https://doc.rust-lang.org/stable/std/io/trait.Read.html#method.read_to_end)を参照のこと。



---

#### std::io::Seek

- Description

  Seekトレイトは、バイトのストリーム内で移動できるカーソルを提供します。

  ストリームは通常、固定サイズであり、終端または現在のオフセットのいずれかに対するシークが可能です。

- Example

  Files implement Seek:

```rust
  use std::io;
  use std::io::prelude::*;
  use std::fs::File;
  use std::io::SeekFrom;
  
  fn main() -> io::Result<()> {
      let mut f = File::open("foo.txt")?;
  
      // move the cursor 42 bytes from the start of the file
      f.seek(SeekFrom::Start(42))?;
      Ok(())
  }
```



---

#### std::io::Seek::seek

- Description

  ストリーム内のオフセット（バイト単位）にシークします。

  ストリームの終端を超えてシークすることは可能ですが、その動作は実装によって定義されます。

  シーク操作が正常に完了した場合、このメソッドは、ストリームの開始点からの新しい位置を返します。この位置は後で [`SeekFrom::Start`](https://doc.rust-lang.org/stable/std/io/enum.SeekFrom.html#variant.Start)で使用できます。

- Errors

  負のオフセットにシークするとエラーになります。

---

#### std::io::SeekFrom

- Description

```rust
  pub enum SeekFrom {
      Start(u64),
      End(i64),
      Current(i64),
  }
```

  I/Oオブジェクト内でシークするために可能なメソッドの列挙。

  これは、[`Seek`](https://doc.rust-lang.org/stable/std/io/trait.Seek.html)トレイトによって使用されます。

- Variants

  - `Start(u64)`

    指定したバイト数のオフセットを設定する

  - `End(i64)`

    オフセットを、このオブジェクトのサイズに指定のバイト数を加えたサイズに設定します。

    オブジェクトの終わりを越えてシークすることは可能ですが、バイト0より前にシークすることはエラーになります。

  - `Current(i64)`

    現在の位置に、指定したバイト数を加えたオフセットを設定します。

    オブジェクトの終わりを越えてシークすることは可能ですが、バイト0より前にシークすることはエラーになります。

---

#### std::io::Write

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

#### std::io::BufRead::lines

  - Description

    Readerの行のイテレータを返す。

    この関数から返されるイテレータは、io::Result<[String]>のインスタンスを返します。返される各文字列は、最後に改行バイト（0xAバイト）やCRLF（0xD、0xAバイト）は持たない。

---

#### std::io::Write::write_all

- Description

  バッファ全体をこのライターに書き込もうとします。

  このメソッドは、書き込むデータがなくなるか、`ErrorKind::Interrupted`以外の種類のエラーが返されるまで、継続して`write`を呼び出します。このメソッドは、バッファ全体が正常に書き込まれるか、そのようなエラーが発生するまで戻りません。このメソッドから生成された`ErrorKind::Interrupted`以外の種類の最初のエラーが返されます。

  バッファにデータが含まれていない場合、このメソッドは決して`write`を呼び出しません。

- Errors

  この関数は、`ErrorKind::Interrupted`以外の種類のエラーで、`write`が返す最初のエラーを返します。

- Example

```rust
  use std::io::prelude::*;
  use std::fs::File;
  
  fn main() -> std::io::Result<()> {
      let mut buffer = File::create("foo.txt")?;
  
      buffer.write_all(b"some bytes")?;
      Ok(())
  }
```
