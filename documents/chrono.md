# Crate chrono

これは、時間ライブラリの特徴完全なスーパーセットであることを目指しています。具体的には

- ChronoはISO 8601に厳密に準拠しています。
- Chronoはデフォルトでタイムゾーンを意識しており、タイムゾーンを意識しない型も用意されています。
- Chronoは空間的に最適化されており、(主目的ではないが)適度に効率的である。

Rustに優れた日付と時刻のライブラリを導入しようとする試みは以前にもいくつかありましたが、Chronoはそれらを基に構築されており、評価されるべきものです。

- `wiki`での初期研究
- Dietrich Eppの`datetime-rs`
- Luis de Bethencourt氏の`rust-datetime`

Chronoへの重要な変更は、[`CHANGELOG.md`](https://github.com/chronotope/chrono/blob/main/CHANGELOG.md)ファイルに記録されます。

---

#### Usage

`Cargo.toml`に以下を記述してください。

```toml
[dependencies]
chrono = "0.4"
```

---

### Features

Chronoは様々なランタイム環境やOSに対応しており、いくつかの機能を有効または無効にすることができます。

デフォルトの機能

- `alloc`: アロケーションに依存する機能（主に文字列のフォーマット）を有効にする
- `std`: 標準ライブラリに依存する機能を有効にします。これはallocのスーパーセットで、標準ライブラリの型や特性との相互運用性が追加されています。
- `clock`: `std::time::SystemTime`が存在するかどうかに関わらず、システムタイム（`now`）の読み取りを可能にします。これはlibcが存在するかどうかに依存します。

オプション機能

- `wasmbind`:wasm-bindgen とその js-sys プロジェクトとの統合を可能にします。
- `serde`:serde によるシリアライズ/デシリアライズを有効にします。
- `unstable-locales`:地域化を有効にします。これにより、様々なメソッドに`_localized`サフィックスが追加されます。パッチリリースでは、実装や API が変更されたり、削除されたりする可能性があります。フィードバックを歓迎します。

機能を指定する例は cargo のドキュメントを参照してください。

---

### Overview

#### Duration

Chronoでは現在、時間軸の大きさを表すために独自の`Duration`型を使用しています。この型は、新しい標準的な継続時間の型と同じ名前であるため、リファレンスではこの型を`OldDuration`と呼ぶことにする。

これは、秒やナノ秒で表現される「正確な」継続時間であり、日や月などの「名目上の」構成要素は表現しないことに注意する。

oldtime機能が有効な場合、`Duration`はtimeクレートのv0.1の`time::Duration`型の別名です。timeクレートのv0.1は非推奨なので、新しいコードでは`oldtime`機能を無効にし、代わりに`chrono::Duration`型を使用してください。`oldtime`機能は後方互換性のためにデフォルトで有効になっていますが、`Chrono`の将来のバージョンでは、この機能は完全に削除される可能性があります。

`Chrono`はまだ標準的なDurationタイプをネイティブにサポートしていませんが、将来的にはサポートされる予定です。一方、`Duration::from_std`および`Duration::to_std`メソッドで2つのタイプ間の変換が可能です。

#### Date and Time

Chronoは、日付とタイムゾーンの時間を表すために、DateTime型を提供しています。

タイムゾーンを意識しない内部計時のような、より抽象的な瞬間の追跡には、システムクロックを追跡するtime::SystemTimeや、不透明だが単調に増加する瞬間の表現であるtime::Instantが適しています。
DateTimeはタイムゾーンを意識しており、ローカルの日付がUTCの日付にどのように変換されるかを定義するTimeZoneオブジェクトから構築する必要があります。TimeZoneには3つの有名な実装があります。

- `Utc`は、`UTC`タイムゾーンを指定します。これは最も効率的です。
- `Local`は、システムのローカルタイムゾーンを指定します。
- `FixedOffset`は、UTC+09:00やUTC-10:30のような任意の固定タイムゾーンを指定します。これは、テキストの日付と時刻を解析した結果であることが多いです。最も多くの情報を格納し、システム環境に依存しないため、他の`TimeZones`をこのタイプに正規化したいと思うでしょう。

しかし、`DateTime::with_timezone`メソッドを使って相互に変換することができます。

現在の日付と時刻は、UTCタイムゾーン(`Utc::now()`)またはローカルタイムゾーン(`Local::now()`)で取得できます。

```rust
use chrono::prelude::*;

let utc: DateTime<Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
```

あるいは、独自の日付と時刻を作成することもできます。Rustでは関数やメソッドのオーバーロードができないため、少し冗長になりますが、その代わりに初期化メソッドの豊富な組み合わせが得られます。

```rust
use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms(9, 10, 11));
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms(9, 10, 11));

let dt = Utc.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, Utc.ymd(2014, 7, 8).and_hms_micro(9, 10, 11, 12_000));
assert_eq!(dt, Utc.ymd(2014, 7, 8).and_hms_nano(9, 10, 11, 12_000_000));

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.ymd(2014, 7, 8).and_hms(21, 15, 33)));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.ymd(2014, 7, 8).and_hms_milli(9, 10, 11, 12);
let fixed_dt = FixedOffset::east(9 * 3600).ymd(2014, 7, 8).and_hms_milli(18, 10, 11, 12);
assert_eq!(dt, fixed_dt);
```

日付と時刻には様々なプロパティが用意されており、個別に変更することができます。これらのプロパティのほとんどは`Datelike`と`Timelike`というトレイトで定義されていて、前に使ったことがあるはずです。加算や減算もサポートされています。以下に、日付と時刻に対してサポートされているほとんどの操作を示します。

```rust
use chrono::prelude::*;
use chrono::Duration;

// assume this returned `2014-11-28T21:45:59.324310806+09:00`:
let dt = FixedOffset::east(9*3600).ymd(2014, 11, 28).and_hms_nano(21, 45, 59, 324310806);

// property accessors
assert_eq!((dt.year(), dt.month(), dt.day()), (2014, 11, 28));
assert_eq!((dt.month0(), dt.day0()), (10, 27)); // for unfortunate souls
assert_eq!((dt.hour(), dt.minute(), dt.second()), (21, 45, 59));
assert_eq!(dt.weekday(), Weekday::Fri);
assert_eq!(dt.weekday().number_from_monday(), 5); // Mon=1, ..., Sun=7
assert_eq!(dt.ordinal(), 332); // the day of year
assert_eq!(dt.num_days_from_ce(), 735565); // the number of days from and including Jan 1, 1

// time zone accessor and manipulation
assert_eq!(dt.offset().fix().local_minus_utc(), 9 * 3600);
assert_eq!(dt.timezone(), FixedOffset::east(9 * 3600));
assert_eq!(dt.with_timezone(&Utc), Utc.ymd(2014, 11, 28).and_hms_nano(12, 45, 59, 324310806));

// a sample of property manipulations (validates dynamically)
assert_eq!(dt.with_day(29).unwrap().weekday(), Weekday::Sat); // 2014-11-29 is Saturday
assert_eq!(dt.with_day(32), None);
assert_eq!(dt.with_year(-300).unwrap().num_days_from_ce(), -109606); // November 29, 301 BCE

// arithmetic operations
let dt1 = Utc.ymd(2014, 11, 14).and_hms(8, 9, 10);
let dt2 = Utc.ymd(2014, 11, 14).and_hms(10, 9, 8);
assert_eq!(dt1.signed_duration_since(dt2), Duration::seconds(-2 * 3600 + 2));
assert_eq!(dt2.signed_duration_since(dt1), Duration::seconds(2 * 3600 - 2));
assert_eq!(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) + Duration::seconds(1_000_000_000),
           Utc.ymd(2001, 9, 9).and_hms(1, 46, 40));
assert_eq!(Utc.ymd(1970, 1, 1).and_hms(0, 0, 0) - Duration::seconds(1_000_000_000),
           Utc.ymd(1938, 4, 24).and_hms(22, 13, 20));
```

#### Formatting and Parsing


フォーマットは`format`メソッドで行われ、`format`はおなじみの`strftime`フォーマットと同等です。

完全な構文と指定子のリストについては、`format::strftime`のドキュメントを参照してください。

デフォルトの`to_string`メソッドと`{:?}`指定子でも、適切な表現が可能です。Chronoは、よく知られたフォーマットのために、`to_rfc2822`と`to_rfc3339`メソッドも提供しています。

クロノは、ほとんどすべての言語での日付フォーマットを、追加のCライブラリなしで提供します。この機能は、`unstable-locales`という機能の下にあります。

```
chrono { version = "0.4", features = ["unstable-locales"]
```

`unstable-locales`の機能は、少なくとも`alloc`の機能を必要とし、それを示唆しています。

```rust
use chrono::prelude::*;

let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2014-11-28 12:00:09");
assert_eq!(dt.format("%a %b %e %T %Y").to_string(), "Fri Nov 28 12:00:09 2014");
assert_eq!(dt.format_localized("%A %e %B %Y, %T", Locale::fr_BE).to_string(), "vendredi 28 novembre 2014, 12:00:09");
assert_eq!(dt.format("%a %b %e %T %Y").to_string(), dt.format("%c").to_string());

assert_eq!(dt.to_string(), "2014-11-28 12:00:09 UTC");
assert_eq!(dt.to_rfc2822(), "Fri, 28 Nov 2014 12:00:09 +0000");
assert_eq!(dt.to_rfc3339(), "2014-11-28T12:00:09+00:00");
assert_eq!(format!("{:?}", dt), "2014-11-28T12:00:09Z");

// Note that milli/nanoseconds are only printed if they are non-zero
let dt_nano = Utc.ymd(2014, 11, 28).and_hms_nano(12, 0, 9, 1);
assert_eq!(format!("{:?}", dt_nano), "2014-11-28T12:00:09.000000001Z");
```

構文解析には3つの方法があります。

1. 標準の`FromStr`トレイト（および文字列に対する`parse`メソッド）は、`DateTime<FixedOffset>`、`DateTime<Utc>`、`DateTime<Local>`の値を解析するために使用できます。これは、`{:?}`の内容を解析します。(`std::fmt::Debug`) フォーマット指定子が印字するものを解析し、オフセットが存在することを必要とします。

2. `DateTime::parse_from_str`はオフセットを含む日付と時刻を解析し、`DateTime<FixedOffset>`を返します。これは、オフセットが入力の一部であり、呼び出し側がそれを推測できない場合に使用する必要があります。オフセットが欠落している可能性がある場合には使用できません。`DateTime::parse_from_rfc2822`と`DateTime::parse_from_rfc3339`は似ていますが、よく知られたフォーマットのためのものです。

3. `Offset::datetime_from_str`も同様ですが、与えられたオフセットの`DateTime`を返します。明示的なオフセットが入力にない場合は、与えられたオフセットを単純に使用します。入力に現在のオフセットとは異なる明示的なオフセットが含まれている場合は、エラーを発行します。

解析プロセスのより詳細な制御は，`format`モジュールで可能です。

```rust
use chrono::prelude::*;

let dt = Utc.ymd(2014, 11, 28).and_hms(12, 0, 9);
let fixed_dt = dt.with_timezone(&FixedOffset::east(9*3600));

// method 1
assert_eq!("2014-11-28T12:00:09Z".parse::<DateTime<Utc>>(), Ok(dt.clone()));
assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<Utc>>(), Ok(dt.clone()));
assert_eq!("2014-11-28T21:00:09+09:00".parse::<DateTime<FixedOffset>>(), Ok(fixed_dt.clone()));

// method 2
assert_eq!(DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"),
           Ok(fixed_dt.clone()));
assert_eq!(DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900"),
           Ok(fixed_dt.clone()));
assert_eq!(DateTime::parse_from_rfc3339("2014-11-28T21:00:09+09:00"), Ok(fixed_dt.clone()));

// method 3
assert_eq!(Utc.datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S"), Ok(dt.clone()));
assert_eq!(Utc.datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y"), Ok(dt.clone()));

// oops, the year is missing!
assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T %Y").is_err());
// oops, the format string does not include the year at all!
assert!(Utc.datetime_from_str("Fri Nov 28 12:00:09", "%a %b %e %T").is_err());
// oops, the weekday is incorrect!
assert!(Utc.datetime_from_str("Sat Nov 28 12:00:09 2014", "%a %b %e %T %Y").is_err());
```

Again : 完全な構文と指定子のリストについては、`format::strftime`のドキュメントを参照してください。

#### Conversion from and to EPOCH timestamps

`Utc.timestamp(seconds, nanoseconds)`を使用して、UNIXタイムスタンプ(1970年1月1日からの経過秒数、ナノ秒数)から`DateTime<Utc>`を構築します。

`DateTime`からタイムスタンプ（秒単位）を取得するには、`DateTime.timestamp`を使用します。さらに、`DateTime.timestamp_subsec_nanos`を使用して、追加のナノ秒の数を得ることができます。

```rust
// We need the trait in scope to use Utc::timestamp().
use chrono::{DateTime, TimeZone, Utc};

// Construct a datetime from epoch:
let dt = Utc.timestamp(1_500_000_000, 0);
assert_eq!(dt.to_rfc2822(), "Fri, 14 Jul 2017 02:40:00 +0000");

// Get epoch value from a datetime:
let dt = DateTime::parse_from_rfc2822("Fri, 14 Jul 2017 02:40:00 +0000").unwrap();
assert_eq!(dt.timestamp(), 1_500_000_000);
```

#### Individual date

Chronoには、個別の日付タイプ（`Date`）も用意されています。このタイプにはタイムゾーンが付随しており、タイムゾーンを介して構築する必要があります。`DateTime`で使用できるほとんどの操作は、必要に応じて`Date`でも使用できます。

```rust
use chrono::prelude::*;
use chrono::offset::LocalResult;

assert_eq!(Utc::today(), Utc::now().date());
assert_eq!(Local::today(), Local::now().date());

assert_eq!(Utc.ymd(2014, 11, 28).weekday(), Weekday::Fri);
assert_eq!(Utc.ymd_opt(2014, 11, 31), LocalResult::None);
assert_eq!(Utc.ymd(2014, 11, 28).and_hms_milli(7, 8, 9, 10).format("%H%M%S").to_string(),
           "070809");
```

#### Naive date and time

Chronoは、`Date`、(存在しない)`Time`、`DateTime`に対応するナイーブな機能を、それぞれ`NaiveDate`、`NaiveTime`、`NaiveDateTime`として提供しています。

これらはタイムゾーン対応のものとほぼ同等のインターフェースを持っていますが、明らかにタイムゾーンに関連付けられておらず、非常に低レベルなものになります。これらは主に、より高いレベルの型の構成要素として役立ちます。

`naive_local`はローカル時間の表示を、`naive_utc`はUTC時間の表示を行います。

#### Limitations

proleptic Gregorian calendar (古い日付をサポートするために拡張されたもの)のみがサポートされています。20世紀以前の日付を扱わなければならない場合は、ユリウス暦やその他の暦の場合もありますので、十分注意してください。

日付の種類は、共通のエポックから約±262,000年に制限されています。時間の種類は、ナノ秒単位の精度に制限されています。

うるう秒は表現上はサポートされていますが、Chronoはそれを利用しようとはしません。(主な理由は、うるう秒は実際には予測できないからです。) 可能なうるう秒に対するほとんどすべての演算は、うるう秒を無視します。必要であれば、暗黙のTAI(International Atomic Time)スケールを持つ`NaiveDateTime`の使用を検討してください。

Chronoは本質的に、不正確な、または部分的な日付と時刻の表現をサポートしていません。曖昧になりうる操作は、そのような場合には`None`を返します。例えば、2014-01-30の "a month later "はうまく定義されておらず、結果として`Utc.ymd(2014, 1, 30).with_month(2)`は`None`を返します。

非 ISO 週の処理はまだサポートされていません。現時点では`chrono_ext`クレート(sources) を使用することができます。

高度なタイムゾーンの処理はまだサポートされていません。今のところ、代わりに`Chrono-tz`クレートをお試しください。

---

### chrono::offset::Local

- Description

  ローカルなタイムスケール。これは、標準のタイムクレートで実装されています。

  `DateTime<Local>`インスタンスを作成するには、`Local`構造体の`TimeZone`メソッドを使用するのが望ましい方法です。

- Example

```rust
use chrono::{Local, DateTime, TimeZone};

let dt: DateTime<Local> = Local::now();
let dt: DateTime<Local> = Local.timestamp(0, 0);
```

#### chrono::offset::Local::now

- Description

  現在の日付に対応する`DateTime`を返します。

### chrono::Datetime

#### chrono::Datetime::format

- Description

  指定されたフォーマット文字列で日付と時刻の組み合わせをフォーマットします。サポートされているエスケープシーケンスについては、`format::strftime`モジュールを参照してください。