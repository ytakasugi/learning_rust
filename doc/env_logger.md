### [env_logger](https://docs.rs/env_logger/0.8.4/env_logger/)

- Description

  環境変数で設定できるシンプルなロガーで、[logクレート](https://docs.rs/log/)が提供するロギング・ファサードで使用します。

  名前に「env」が付いていますが、`env_logger`は環境変数以外の方法で設定することもできます。詳しい方法については、ソースリポジトリの[例](https://github.com/env-logger-rs/env_logger/tree/master/examples)を参照してください。

  デフォルトでは、`env_logger`はログをstderrに書き込みますが、代わりに`stdout`に書き込むように設定することもできます。

  ---

- Example

  ```rust
  use log::{debug, error, log_enabled, info, Level};
  
  env_logger::init();
  
  debug!("this is a debug {}", "message");
  error!("this is printed by default");
  
  if log_enabled!(Level::Info) {
      let x = 3 * 4; // expensive computation
      info!("the answer was: {}", x);
  }
  ```

  バイナリが`main.rs`であることを想定しています。

  ```{.bash}
  $ RUST_LOG=error ./main
  [2017-11-09T02:12:24Z ERROR main] this is printed by default
  ```

  ```{.bash}
  $ RUST_LOG=info ./main
  [2017-11-09T02:12:24Z ERROR main] this is printed by default
  [2017-11-09T02:12:24Z INFO main] the answer was: 12
  ```

  ```{.bash}
  $ RUST_LOG=debug ./main
  [2017-11-09T02:12:24Z DEBUG main] this is a debug message
  [2017-11-09T02:12:24Z ERROR main] this is printed by default
  [2017-11-09T02:12:24Z INFO main] the answer was: 12
  ```

  また、モジュールごとにログレベルを設定することもできます。

  ```{.bash}
  $ RUST_LOG=main=info ./main
  [2017-11-09T02:12:24Z ERROR main] this is printed by default
  [2017-11-09T02:12:24Z INFO main] the answer was: 12
  ```

  そして、すべてのログを有効にします。

  ```{.bash}
  $ RUST_LOG=main ./main
  [2017-11-09T02:12:24Z DEBUG main] this is a debug message
  [2017-11-09T02:12:24Z ERROR main] this is printed by default
  [2017-11-09T02:12:24Z INFO main] the answer was: 12
  ```

  バイナリ名に「-」が含まれている場合は、「_」に置き換える必要があります。

  ```{.bash}
  $ RUST_LOG=my_app ./my-app
  [2017-11-09T02:12:24Z DEBUG my_app] this is a debug message
  [2017-11-09T02:12:24Z ERROR my_app] this is printed by default
  [2017-11-09T02:12:24Z INFO my_app] the answer was: 12
  ```

  これは、Rustのモジュールとクレートは名前にハイフンを含むことができないためですが、cargoは引き続きハイフンを受け入れます。

  `API`の詳細については、[logクレート](https://docs.rs/log/)のドキュメントを参照してください。
  
  ---
  
- Enabling logging

  ログレベルはモジュールごとに制御され、デフォルトではエラーレベル以外のすべてのログは無効になっています。

  ロギングは`RUST_LOG`環境変数で制御されます。この環境変数の値は、コンマで区切られたログ記録ディレクティブのリストです。ロギングディレクティブは次のような形式です。

  ```text
  path::to::module=level
  ```

  モジュールへのパスは、そのモジュールがコンパイルされたクレートの名前を基にしています。たとえば、プログラムが`hello.rs`というファイルに含まれている場合、このファイルのロギングをオンにするには、`RUST_LOG=hello`という値を使用します。さらに、このパスはプレフィックス検索なので、指定されたモジュールにネストされたすべてのモジュールでもロギングが有効になります。

  クレート名やモジュールのパスを指定する場合、ログレベルの明示的な指定はオプションです。省略すると、そのアイテム（およびその子）のすべてのロギングが有効になります。

  指定できるログレベルの名前は、ログクレートの[`log::Level`](https://docs.rs/log/latest/log/enum.Level.html)列挙のバリエーションに対応しています。これらは次のとおりです。

  - `error`
  - `warn`
  - `info`
  - `debug`
  - `trace`

  また、擬似的なロギングレベルとしてoffがあり、これを指定することで、特定のモジュールまたはアプリケーション全体のロギングをすべて無効にすることができます。ロギングレベルと同様に、大文字と小文字の区別はありません1。

  例えば、`debug`、`DEBUG`、`dEbuG`はすべて同じロギングレベルを表します。一貫性を保つために、私たちは小文字の名前を使うことにしています。ドキュメントでは他の形式を使用していますが、それは特定の例の文脈の中で行われているので、自然界で同様の使用法を目にしても驚くことはありません。

  モジュールのログレベルは任意であるため、ログを有効にするモジュールも任意です。レベルのみが指定された場合は、すべてのモジュールのグローバルなログレベルがこの値に設定されます。

  `RUST_LOG`の有効な値の例をいくつか示します。

  - `hello` "hello"モジュールのログをすべてオンにする
  - `trace` アプリケーションの名前にかかわらず、そのアプリケーションのすべてのログをオンにします。
  - `TRACE` アプリケーションの名前にかかわらず、そのアプリケーションのすべてのログをオンにする（前と同じ)
  - `info`は、すべての情報ログをオンにする
  - `INFO`は、すべての情報ログをオンにする（前回と同じ）。
  - `hello=debug`は、"hello"のデバッグログをオンにする
  - `hello=DEBUG`は、"hello"のデバッグログをオンにする（前回と同じ）。
  - `hello,std::option`は、`hello`と`std`のオプションロギングをオンにします。
  - `error,hello=warn`は、グローバルなエラーログをオンにして、helloの警告も出す
  - `error,hello=off`は、グローバルなエラーログをオンにするが、helloのログをオフにする
  - `off`は、アプリケーションのすべてのログをオフにする
  - `OFF`は、アプリケーションのすべてのログをオフにする（前と同じ)

  ---

- Filtering results

  `RUST_LOG`ディレクティブには、正規表現フィルタを含めることができます。構文は`/`の後に正規表現を加えたものです。各メッセージは正規表現と照合され、一致した場合のみログに記録されます。マッチングはログの文字列をフォーマットした後、ログのメタデータを追加する前に行われることに注意してください。すべてのモジュールに対して、単一のフィルタがあります。

  - `hello/foo`は、'hello' モジュールのログメッセージに 'foo' が含まれている場合、すべてのログをオンにします。
  - `info/f.o`は、ログメッセージに「foo」、「f1o」、「fao」などが含まれる場合、すべての情報ログをオンにします。
  - `hello=debug/foo*foo`は、 「hello」でログメッセージに「foofoo」や「fofoo」、「fooooofoo」などが含まれている場合、デバッグログをオンにします。
  - `error,hello=warn/[0-9]scopes`は、 グローバルなエラーログをオンにし、さらに`hello`の警告を行います。どちらの場合も、ログメッセージには、一桁の数字の後に「scopes」を付けなければなりません。

  ---

- Capturing logs in tests

   `cargo test` 中に記録されたレコードは、デフォルトではテストハーネスに取り込まれません。[`Builder::is_test`](https://docs.rs/env_logger/0.8.4/env_logger/struct.Builder.html#method.is_test)メソッドをユニットテストで使用することで、ログが確実に取得されるようになります。

  ```rust
  #[cfg(test)]
  mod tests {
      fn init() {
          let _ = env_logger::builder().is_test(true).try_init();
      }
  
      #[test]
      fn it_works() {
          init();
  
          info!("This record will be captured by `cargo test`");
  
          assert_eq!(2, 1 + 1);
      }
  }
  ```

  テストキャプチャーを可能にすると、カラーやその他のスタイルのサポートが犠牲になり、パフォーマンスに影響が出ることがあります。

  ---

- Disabling colors

  色やその他のスタイルは、環境変数`RUST_LOG_STYLE`で設定できます。環境変数には次のような値があります。

  - `auto`(デフォルト) は、スタイル文字の印刷を試みますが、強制はしません。Windowsでコンソールが利用できない場合や、`TERM=dumb`の場合などは、色を表示しません。
  - `always`は、スタイル文字がターミナルでサポートされていなくても、常にスタイル文字を表示します。これには、Windows でコンソール API が利用できない場合に ANSI カラーを表示することも含まれます。
  - `never`は、決してスタイル文字を表示しません。

  ---

- Tweaking the default format

  [`Builder`](https://docs.rs/env_logger/0.8.4/env_logger/struct.Builder.html)を使って、デフォルトフォーマットの一部をログ出力から除外することができます。以下の例では、ログ出力からタイムスタンプを除外しています。

  ```rust
  env_logger::builder()
      .format_timestamp(None)
      .init();
  ```

  ---

- Stability of the default format

  デフォルトのフォーマットは長期的な安定性のために最適化されておらず、`0.x`の間のメジャー、マイナー、パッチのバージョンの違いに関わらず、出力の安定性を明示的に保証するものではありません。

  `env_logger`の出力をプログラムで取得したり解釈したりしたい場合は、カスタムフォーマットを使用してください。

  ---

- Using a custom format

  カスタムフォーマットはBuilderのクロージャとして提供することができます。これらのクロージャは、[`Formatter`](https://docs.rs/env_logger/0.8.4/env_logger/fmt/struct.Formatter.html)と`log::Record`を引数として取ります。

  ```rust
  use std::io::Write;
  
  env_logger::builder()
      .format(|buf, record| {
          writeln!(buf, "{}: {}", record.level(), record.args())
      })
      .init();
  ```

  カスタムフォーマットの詳細については、[`fmt`](https://docs.rs/env_logger/0.8.4/env_logger/fmt/index.html)モジュールを参照してください。

  ---

- Specifying defaults for environment variables

  `env_logger`は、環境変数から設定を読み取ることができます。これらの変数が存在しない場合、使用するデフォルト値は[`Env`](https://docs.rs/env_logger/0.8.4/env_logger/struct.Env.html)タイプで調整することができます。次の例では、環境変数`RUST_LOG`が設定されていない場合、デフォルトで警告以上のログを記録します。

  ```rust
  use env_logger::Env;
  
  env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
  
  ```

  1. ログレベル名の宇宙と同様に、オフの疑似ログレベル機能も基礎となるログクレートによって提供されます。

---

### env_logger::init

- Description

  グローバルロガーをenvロガーで初期化します。

  これは、Rustプログラムの実行の初期に呼び出されるべきです。初期化の前に発生したログイベントは無視されます。

- Panic

  この関数は、複数回呼ばれたり、他のライブラリがグローバルロガーをすでに初期化していたりすると、パニックを起こします。

