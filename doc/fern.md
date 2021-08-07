# Crate fern

効率的で設定可能なRustのロギング

### Depending on fern

プロジェクトのCargo.tomlにfernとlogの両方が必要であるため、依存関係に追加されていることを確認してください。

```toml
[dependencies]
log = "0.4"
fern = "0.5"
```

### Example setup

fernでは、すべてのロガーの設定は、`Dispatch`構造体のインスタンス上のビルダーのようなメソッドを介して行われます。

ここでは、メッセージをフォーマットし、`Debug`以上のメッセージを標準出力と`output.log`ファイルの両方に送信するロガーの例を示します。

```rust
use log::{debug, error, info, trace, warn};

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
```

これを解いてみましょう。

[`fern::Dispatch::new()`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html#method.new)

空のコンフィグレーションを作成します。

------

[`.format(|...| ...)`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html#method.format)

ロガーにフォーマッターを追加し、送信されるすべてのメッセージを修正します。

------

[`chrono::Local::now()`](https://docs.rs/chrono/0.4/chrono/offset/local/struct.Local.html#method.now)

 [`chrono`](https://github.com/chronotope/chrono)を使って、ローカルタイムゾーンの現在時刻を取得します。
 
 See the [time-and-date docs](https://docs.rs/chrono/0.4/chrono/index.html#date-and-time).

------

[`.format("[%Y-%m-%d\][%H:%M:%S]")`](https://docs.rs/chrono/0.4/chrono/datetime/struct.DateTime.html#method.format)

`chrono`の遅延フォーマット指定子を使って、時間を読みやすい文字列に変換します。

------

[`out.finish(format_args!(...))`](https://docs.rs/fern/0.6.0/fern/struct.FormatCallback.html#method.finish)

フォーマットされたメッセージを送信するために、`fern::FormattingCallback`を呼び出します。

この遠回りな方法は少し変ですが、非常に高速なロギングが可能になります。文字列の割り当ては必要ありません。

[`format_args!()`](https://doc.rust-lang.org/std/macro.format_args.html) と [`println!()`](https://doc.rust-lang.org/std/macro.println.html) (および、ほかのすべての [`std::fmt`](https://doc.rust-lang.org/std/fmt/)ベースのマクロ)同じフォーマットです。

------

[`.level(log::LevelFilter::Debug)`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html#method.level)

「デバッグ」に出力するための最小レベルを設定します。

------

[`.chain(std::io::stdout())`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html#method.chain)

loggerに子機を追加します。フィルターを通過したメッセージはすべて標準出力に送られます。

[`Dispatch::chain`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html#method.chain)は、 [`Stdout`](https://doc.rust-lang.org/std/io/struct.Stdout.html), [`Stderr`](https://doc.rust-lang.org/std/io/struct.Stderr.html), [`File`](https://doc.rust-lang.org/std/fs/struct.File.html)やその他の[`Dispatch`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html)インスタンスを受けれ入れます。

------

[`.chain(fern::log_file(...)?)`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html#method.chain)

Aファイル "output.log "にメッセージを送信する2番目の子を追加します。

ファイル出力の詳細については、[`fern::log_file()`](https://docs.rs/fern/0.6.0/fern/fn.log_file.html)を参照してください。

------

[`.apply()`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html#method.apply)

設定を取り込み、現在のランタイムのグローバルロガーとしてインスタンス化します。

これは、`.apply()`または同等のフォームの他のクレートがすでにこのランタイムで使用されている場合に限り、失敗します。

バイナリクレートはロギングを設定する唯一のものなので、[`apply`](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html#method.apply)の結果は合理的に解きほぐすことができます。誰かがこのメソッドを2回以上呼び出しているクレートはバグです。

------

最終的な出力は以下のようになります。

```text
[2017-01-20][12:55:04][crate-name][INFO] Hello, world!
[2017-01-20][12:56:21][crate-name][WARN] Ahhh!
[2017-01-20][12:58:00][crate-name][DEBUG] Something less important happened.
```

### Logging

ロガーが設定されると、クレートや依存しているすべてのライブラリからのすべてのロギングコールを拾います。

```rust

fern::Dispatch::new()
    // ...
    .apply()?;

trace!("Trace message");
debug!("Debug message");
info!("Info message");
warn!("Warning message");
error!("Error message");
```

### More

[`Dispatch`のドキュメント](https://docs.rs/fern/0.6.0/fern/struct.Dispatch.html)には、各メソッドの使用例があり、[完全なサンプルプログラム](https://github.com/daboross/fern/tree/master/examples/cmd-program.rs)は、より大きなアプリケーションのコンテキストで`fern`を使用するのに役立つかもしれません。

ANSIターミナルカラーリングの例は、[`colors`](https://docs.rs/fern/0.6.0/fern/colors/index.html)モジュールを参照してください。

unixの[`syslog`](https://docs.rs/fern/0.6.0/fern/syslog/index.html)に出力する例については`syslog`モジュールを、より現実的なサンプルとしては[`syslog`のフルサンプルプログラム](https://github.com/daboross/fern/tree/master/examples/syslog.rs)を参照してください。

`log-within-logging`を正しく動作させるための情報については [`meta`](https://docs.rs/fern/0.6.0/fern/meta/index.html)モジュールを参照してください。