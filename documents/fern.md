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

ファイル "output.log "にメッセージを送信する2番目の子を追加します。

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

---

### fern::Dispatch

- Description

  ベースとなるディスパッチロガーです。

  これにより、ログレコードをフォーマットしたり、通過させるレコードを制限したり、他のディスパッチロガーや出力ロガーにレコードをディスパッチしたりすることができます。

  すべてのメソッドは位置を区別しないことに注意してください。`Dispatch::new().format(a).chain(b)`は、`Dispatch::new().chain(b).format(a)`とまったく同じ結果になります。このことを考えると、わかりやすくするために、`format`や他の修飾語を`chain`の前に置くことが望ましい。

- Example

```rust
use std::{fs, io};

fern::Dispatch::new()
    .format(|out, message, record| {
        out.finish(format_args!(
            "[{}][{}] {}",
            record.level(),
            record.target(),
            message,
        ))
    })
    .chain(
        fern::Dispatch::new()
            // デフォルトでは警告メッセージのみ受け付けます。
            .level(log::LevelFilter::Warn)
            // 現在のクレートからの情報メッセージも受け取る
            .level_for("my_crate", log::LevelFilter::Info)
            // io::Stdout`, `io::Stderr`, `io::File` を直接渡すことができます。
            .chain(io::stdout()),
    )
    .chain(
        fern::Dispatch::new()
            // 全メッセージの出力
            .level(log::LevelFilter::Trace)
            // hyper以外の場合は、情報メッセージのみ表示されます。
            .level_for("hyper", log::LevelFilter::Info)
            // log_file(x)`は、`OpenOptions::new().write(true).append(true).create(true).open(x)`と同じです。
            .chain(fern::log_file("persistent-log.log")?)
            .chain(
                fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .create(true)
                    .open("/tmp/temp.log")?,
            ),
    )
    .chain(
        fern::Dispatch::new()
            .level(log::LevelFilter::Error)
            .filter(|_meta_data| {
                // 例として、メッセージの半分をランダムに拒否すると
                rand::random()
            })
            .chain(io::stderr()),
    )
    // そして最後に、グローバルロガーとして設定します。
    .apply()?;
```

#### fern::DisPatch::format

- Description

  このディスパッチのフォーマッタを設定します。クロージャは、コールバック、メッセージ、ログレコードを受け取り、結果のフォーマットをライターに書き込む必要があります。

`record.args()`は元のログメッセージを取得するために使用することができますが、真のログチェイニングを可能にするために、フォーマッタは出力にメッセージを含める際には必ず与えられたメッセージを使用しなければなりません。

中間結果のすべての割り当てを避けるために、フォーマッタはコールバックを呼び出すことで「完了」し、コールバックは新しいフォーマットされたメッセージでロギングチェーンの残りの部分を呼び出します。コールバックオブジェクトは、スタックのブール値によって呼び出されたかどうかを追跡するので、out.finishを使用しない場合、ログメッセージはフォーマットされずにロガーチェーンを進みます。

- Example

```rust
fern::Dispatch::new().format(|out, message, record| {
    out.finish(format_args!(
        "[{}][{}] {}",
        record.level(),
        record.target(),
        message
    ))
})
```

#### fern::DisPatch::level

- Description

  このロガーに包括的なレベルフィルタを設定します。`Dispatch::level_for`で設定されたものによってまだフィルタリングされていないすべてのメッセージが影響を受けます。

  フィルタリングされたすべてのメッセージは、与えられたレベルよりも深刻度が低い場合、破棄されます。

  デフォルトのレベルは`LevelFilter::Trace`です。
  
- Example

```rust
fern::Dispatch::new().level(log::LevelFilter::Info)
```

#### fern::DisPatch::level_for

- Description

  ターゲットごとのログレベルフィルタを設定します。ログメッセージのデフォルトターゲットは、`crate_name::module_name`または`crate_name`で、`crate`ルートのログの場合は`crate_name`です。ターゲットは `info!(target: "target-name", ...)`で設定することもできます。

  各ログレコードに対して、fernはまず最も具体的な`level_for`にマッチしようとし、マッチするレベルが見つかるか、デフォルトのレベルが使用されるまで、徐々に一般的なものになっていきます。

  例えば、ターゲットである`hyper::http::h1`のログは、最初に`hyper::http::h1`の`level_for`をテストし、次に`hyper::http`をテストし、次に`hyper`をテストし、その後、デフォルトのレベルを使用します。

- Example

  あるプログラムが多くのデバッグ出力を含みたいが、ライブラリ "hyper"はうまく動作することが知られているので、そのライブラリからのデバッグ出力は除外する必要がある。

```rust
fern::Dispatch::new()
    .level(log::LevelFilter::Trace)
    .level_for("hyper", log::LevelFilter::Info)
```

  プログラムはモジュールごとに大量のデバッグ出力を持っていますが、あまりにも多すぎるため、一度に複数のモジュールをデバッグしてもあまり意味がありません。このコマンドラインは、プログラムの残りの部分を情報レベルに保ちながら、デバッグするモジュールのリストを受け付けます。

```rust
fn setup_logging<T, I>(verbose_modules: T) -> Result<(), fern::InitError>
where
    I: AsRef<str>,
    T: IntoIterator<Item = I>,
{
    let mut config = fern::Dispatch::new().level(log::LevelFilter::Info);

    for module_name in verbose_modules {
        config = config.level_for(
            format!("my_crate_name::{}", module_name.as_ref()),
            log::LevelFilter::Debug,
        );
    }

    config.chain(std::io::stdout()).apply()?;

    Ok(())
}
```

#### fern::Dispatch::chain

- Description

  このディスパッチに子を追加します。

  すべてのフィルターを通過したすべてのログレコードは、フォーマットされ、すべての子ロガーに順に送信されます。

  注： 子ロガーがディスパッチでもあり、ログレコードを受け入れられない場合は、ドロップされます。これは、子ロガーに子がいないか、最小ログレベルが LevelFilter::Off の場合にのみ起こります。

- Example

```rust
fern::Dispatch::new().chain(fern::Dispatch::new().chain(std::io::stdout()))
```

### fern::FormatCallback

#### fern::FormatCallback::finish

- Description

  この`FormatCallback`が作成されたフォーマット呼び出しを完了します。

  これは、新しいペイロードメッセージとして与えられたフォーマットされたメッセージを使用して、ロギングチェーンの残りの部分を呼び出します。