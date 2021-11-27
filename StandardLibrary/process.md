### std::process

  - Description

    プロセスを扱うモジュール。

    このモジュールは、主に子プロセスの生成と相互作用に関係していますが、現在のプロセスを終了させるための`abort`と `exit]`も提供している。

---

#### std::process::exit

  - Description

    指定した終了コードで現在のプロセスを終了させる。

    この関数は何も返却せず、現在のプロセスを即座に終了させる。終了コードは基盤となるOSに渡され、別のプロセスで使用できるようになる。

    この関数は何も返却せず、プロセスを終了するので、現在のスタックや他のスレッドのスタック上のデストラクタは実行されないことに注意すること。クリーンなシャットダウンが必要な場合は、実行するデストラクタがなくなった時点でのみこの関数を呼び出すことを検討すること。

---

####  std::process::Command

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

#### std::process::Command::spawn

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

#### std::process::Command::output

- Description

  子プロセスとしてコマンドを実行し、コマンドの終了を待ち、その出力をすべて収集します。

  デフォルトでは、`stdout`と`stderr`が収集されます（結果の出力にも使用されます）。`stdin`は親プロセスから継承されず、子プロセスが`stdin`ストリームから読み取ろうとすると、ストリームは直ちに閉じられます。

- Example

```rust
  use std::process::Command;
  use std::io::{self, Write};
  let output = Command::new("/bin/cat")
                       .arg("file.txt")
                       .output()
                       .expect("failed to execute process");
  
  println!("status: {}", output.status);
  io::stdout().write_all(&output.stdout).unwrap();
  io::stderr().write_all(&output.stderr).unwrap();
  
  assert!(output.status.success());
```

  

---

#### std::process::Child

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
