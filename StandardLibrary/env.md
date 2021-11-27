### std::env

---

#### std::env::Args

  - Description

    プロセスの引数に対するイテレータで、各引数の`String`値を返す。
    この構造体は`env::args()`によって作成される。詳細はドキュメントを参照のこと。
    最初の要素は伝統的に実行ファイルのパスですが、任意のテキストを設定することもでき、存在しない場合もある。つまり、このプロパティはセキュリティのために頼るべきではないということである。

---

#### std::env::args

  - Description

    このプログラムが開始されたときの引数（通常はコマンドラインで渡される）を返します。
    
    最初の要素は伝統的には実行ファイルのパスですが、任意のテキストに設定することができ、存在しない可能性もあります。つまり、このプロパティをセキュリティ目的で信頼してはいけないということです。
    
    Unixシステムでは、シェルは通常、引用符で囲まれていない引数をグロブパターン（`*`や`?`など）で展開します。Windowsではこのような処理は行われず、そのような引数はそのまま渡されます。
    
    glibc Linuxシステムでは、引数は、関数を`.init_array`に置くことで取得されます。glibcは、非標準の拡張として、`argc`、`argv`、および`envp`を`.init_array`の関数に渡します。これにより、`cdylib`や`staticlib`であっても、macOSやWindowsのように`std::env::args`が動作するようになります。
    
- Panics

  返されたイテレータは、プロセスへの引数が有効なUnicodeでない場合、反復中にパニックを起こします。これが望ましくない場合は、代わりに`args_os`関数を使用してください。

- Example

```rust
  use std::env;
  
  // Prints each argument on a separate line
  for argument in env::args() {
      println!("{}", argument);
  }
```

  

---

#### std::env::set_current_dir

- Description

  現在の作業ディレクトリを、指定したパスに変更します。

  操作に失敗した場合は、`Err`を返します。

---

#### std::env::var

  - Description

    現在のプロセスから環境変数のキーを取得する。

