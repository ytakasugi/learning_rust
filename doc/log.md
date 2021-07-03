### log

- Description

  軽量のロギングファサードです。

  [logクレート](https://docs.rs/log/0.4.14/log/)は、実際のロギング実装を抽象化した単一のロギングAPIを提供します。ライブラリは、このクレートによって提供されるロギングAPIを使用することができ、それらのライブラリの消費者は、そのユースケースに最も適したロギング実装を選択することができます。

  ロギング実装が選択されない場合、ファサードは、すべてのログメッセージを無視する「noop」実装にフォールバックします。この場合のオーバーヘッドは非常に小さく、整数のロード、比較、ジャンプだけです。

  ログリクエストは、ターゲット、レベル、ボディで構成されます。ターゲットは文字列で、デフォルトではログリクエストの場所のモジュールパスになっていますが、このデフォルトはオーバーライドすることができます。ロガーの実装では、通常、ターゲットを使用して、ユーザーの設定に基づいてリクエストをフィルタリングします。

  ---

- Use

  ログクレートの基本的な使い方は、`error！`、`warn！`、`info！`、`debug！`、`trace！`の5つのログマクロで、`error！`が最も優先度の高いログメッセージ、`trace！`が最も優先度の低いログメッセージを表します。ログメッセージは、優先度の低いメッセージを除外するようにログレベルを設定することでフィルタリングされます。これらのマクロは、`println! `と同様にフォーマット文字列を受け付けます。

  ---

- In libraries

  ライブラリは、[logクレート](https://docs.rs/log/0.4.14/log/)にのみリンクし、提供されたマクロを使用して、下流の消費者にとって有用な情報をログに記録する必要があります。
  
  ---
  
- Example

  ```rust
  use log::{info, warn};
  
  pub fn shave_the_yak(yak: &mut Yak) {
      info!(target: "yak_events", "Commencing yak shaving for {:?}", yak);
  
      loop {
          match find_a_razor() {
              Ok(razor) => {
                  info!("Razor located: {}", razor);
                  yak.shave(razor);
                  break;
              }
              Err(err) => {
                  warn!("Unable to locate a razor: {}, retrying", err);
              }
          }
      }
  }
  ```

  ---

- In executables

  実行ファイルは、プログラムの実行時の早い段階で、ロギングの実装を選択し、それを初期化する必要があります。ロギングの実装には、通常、これを行うための関数が含まれています。実装が初期化される前に生成されたログメッセージはすべて無視されます。

  実行ファイル自身も、ログを取るために[logクレート](https://docs.rs/log/0.4.14/log/)を使用することができます。

  ---

- Warning

  ロギングシステムの初期化は一度だけです。

  ---

- Available logging implementations

  実行ファイルがログを出力するためには、ファサードと互換性のあるロガーの実装を使用する必要があります。多くの利用可能な実装がありますが、ここでは最も人気のあるものをいくつか紹介します。

  - Simple minimal loggers:
    - [env_logger](https://docs.rs/env_logger/*/env_logger/)
    - [simple_logger](https://github.com/borntyping/rust-simple_logger)
    - [simplelog](https://github.com/drakulix/simplelog.rs)
    - [pretty_env_logger](https://docs.rs/pretty_env_logger/*/pretty_env_logger/)
    - [stderrlog](https://docs.rs/stderrlog/*/stderrlog/)
    - [flexi_logger](https://docs.rs/flexi_logger/*/flexi_logger/)
  - Complex configurable frameworks:
    - [log4rs](https://docs.rs/log4rs/*/log4rs/)
    - [fern](https://docs.rs/fern/*/fern/)
  - Adaptors for other facilities:
    - [syslog](https://docs.rs/syslog/*/syslog/)
    - [slog-stdlog](https://docs.rs/slog-stdlog/*/slog_stdlog/)

  ---

- Implementing Logger

  ロガーは、Logトレイトを実装しています。ここでは、[`Error`](https://docs.rs/log/0.4.14/log/enum.Level.html)、 [`Warn`](https://docs.rs/log/0.4.14/log/enum.Level.html) 、 [`Info`](https://docs.rs/log/0.4.14/log/enum.Level.html) レベルのすべてのメッセージを標準出力に記録するだけの非常に基本的な例を示します。

  ```rust
  use log::{Record, Level, Metadata};
  
  struct SimpleLogger;
  
  impl log::Log for SimpleLogger {
      fn enabled(&self, metadata: &Metadata) -> bool {
          metadata.level() <= Level::Info
      }
  
      fn log(&self, record: &Record) {
          if self.enabled(record.metadata()) {
              println!("{} - {}", record.level(), record.args());
          }
      }
  
      fn flush(&self) {}
  }
  ```

  ロガーのインストールは、`set_logger`関数を呼び出して行います。また、最大のログレベルは、`set_max_level`関数で調整する必要があります。ロギング・ファサードは、無効なレベルのログ・メッセージのパフォーマンスを向上させるための最適化としてこれを使用しています。これを設定することが重要です。デフォルトは`Off`で、ログメッセージはキャプチャされません。今回のロガーの例では、`Debug`や`Trace`レベルのログメッセージを無視するため、最大のログレベルを`Info`に設定することになります。ロギングの実装では、[`set_logger`](https://docs.rs/log/0.4.14/log/fn.set_logger.html) と[`set_max_level`](https://docs.rs/log/0.4.14/log/fn.set_max_level.html)の呼び出しをラップする関数を提供し、ロガーの初期化を処理する必要があります。

  ```rust
  use log::{SetLoggerError, LevelFilter};
  
  static LOGGER: SimpleLogger = SimpleLogger;
  
  pub fn init() -> Result<(), SetLoggerError> {
      log::set_logger(&LOGGER)
          .map(|()| log::set_max_level(LevelFilter::Info))
  }
  ```

  実行時に設定を変更する実装では、最大のログレベルも調整するようにしてください。

  ---

- Use with `std`

  `set_logger`では、`&'static Log`を提供する必要がありますが、ロガーが何らかのランタイム設定に依存している場合には、入手が困難な場合があります。`set_boxed_logger`関数は、`std`Cargo機能で利用できます。`set_logger`と同じですが、`&'static Log`ではなく`Box<Log>`を受け取ることが特徴です。
  
  ```rust
  pub fn init() -> Result<(), SetLoggerError> {
      log::set_boxed_logger(Box::new(SimpleLogger))
          .map(|()| log::set_max_level(LevelFilter::Info))
  }
  ```
  
  ---
  
  - Compile time filters
  
    ログレベルは、Cargoの機能を使ってコンパイル時に静的に無効にすることができます。無効化されたレベルでのログの呼び出しはスキップされ、結果として得られるバイナリにも存在しません。このレベルは、リリースビルドとデバッグビルドで別々に設定されます。
  
    - `max_level_off`
    - `max_level_error`
    - `max_level_warn`
    - `max_level_info`
    - `max_level_debug`
    - `max_level_trace`
    - `release_max_level_off`
    - `release_max_level_error`
    - `release_max_level_warn`
    - `release_max_level_info`
    - `release_max_level_debug`
    - `release_max_level_trace`
  
    これらの機能は、`STATIC_MAX_LEVEL`定数の値を制御します。ログ・マクロは、メッセージを記録する前にこの値をチェックします。デフォルトでは、どのレベルも無効になっています。
  
    最大レベルの機能はグローバルなもので、一度設定すると変更できないため、ライブラリは使用しないようにしてください。
  
    例えば、クレートは次のような設定で、デバッグビルドではトレースレベルのログを、リリースビルドではトレース、デバッグ、インフォレベルのログを無効にすることができます。
  
    ```
    [dependencies]
    log = { version = "0.4", features = ["max_level_debug", "release_max_level_warn"] }
    ```
  
    ---
  
  - Cate Feature Flags
  
    フィルターに加えて、以下のcrate機能フラグが利用できます。これらはあなたの`Cargo.toml`で設定されます。
  
    - `std`は、デフォルトのコアではなく `std`クレートを使用できるようにします。`std::error`と`set_boxed_logger`の機能が使えるようになりました。
    - `serde`は、`Level`と`LevelFilter`のシリアル化とデシリアル化をサポートします。
  
    ```
    [dependencies]
    log = { version = "0.4", features = ["std", "serde"] }
    ```
  
    ---
  
  - Version compatibility
  
    `log` crateの0.3と0.4のバージョンは、ほぼ完全に互換性があります。`log` 0.3を使用して作成されたログメッセージは、`log` 0.4を使用するloggerの実装に透過的に転送されます。`log` 0.4を使用して作成されたログメッセージは `log` 0.3を使用するloggerの実装に転送されますが、メッセージに関連するモジュールパスとファイル名の情報は残念ながら失われます。
  
    
  
  

