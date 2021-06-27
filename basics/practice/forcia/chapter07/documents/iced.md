### [Crate iced](https://docs.rs/iced/0.2.0/iced/)

  - Description
    Iced は、シンプルさと型の安全性に焦点を当てたクロスプラットフォームの GUI ライブラリです。Elm にインスパイアされています。

---

- Feature

  - シンプルで使いやすい、batteries-included API
  - タイプセーフ、リアクティブプログラミングモデル
  - クロスプラットフォーム対応
  - レスポンシブレイアウト
  - 内蔵ウィジェット（テキスト入力、スクロール可能などを含む)
  - カスタムウィジェットのサポート（独自のウィジェットを作成)
  - パフォーマンスメトリクスのデバッグオーバーレイ
  - 非同期アクションのファーストクラスのサポート (futuresを使用してください！)
  - 再利用可能な部分に分割されたモジュラーエコシステム
    - 既存のシステムとの統合を可能にするレンダラーに依存しないネイティブランタイム
    - Vulkan、Metal、DX11、DX12をサポートする内蔵レンダラー
    - ウインドウイングシェル
    - DOM を活用したウェブランタイム

  詳細は、[リポジトリ](https://github.com/hecrj/iced)と[Example](https://github.com/hecrj/iced/tree/0.2/examples)を確認してください！

---

  - Overview

    The Elm ArchitectureにインスパイアされたIcedは、ユーザーインターフェースを4つの異なるコンセプトに分割することを期待しています。

    - State -  アプリケーションの状態
    - Message - ユーザーとのインタラクションや、気になる意味のあるイベント
    - View logic - ユーザーインタラクションでメッセージを生成するウィジェットとして状態を表示する方法
    - Update logic - メッセージに反応して状態を更新する方法

    これがどのように動作するかを確認するために何かを作ることができます 2つのボタンを使ってインクリメントとデクリメントができるシンプルなカウンターが欲しいとしましょう。

    アプリケーションの状態をモデル化することから始めます。

    ~~~rust
    use iced::button;
    
    struct Counter {
        // The counter value
        value: i32,
    
        // The local state of the two buttons
        increment_button: button::State,
        decrement_button: button::State,
    }
    ~~~

    次に、カウンタのユーザーインタラクションを定義する必要があります。これらのインタラクションがメッセージです。

    ~~~rust
    #[derive(Debug, Clone, Copy)]
    pub enum Message {
        IncrementPressed,
        DecrementPressed,
    }
    ~~~

    では、実際のカウンターをビューロジックにまとめて見せてみましょう。

    ~~~rust
    use iced::{Button, Column, Text};
    
    impl Counter {
        pub fn view(&mut self) -> Column<Message> {
            // We use a column: a simple vertical layout
            Column::new()
                .push(
                    // The increment button. We tell it to produce an
                    // `IncrementPressed` message when pressed
                    Button::new(&mut self.increment_button, Text::new("+"))
                        .on_press(Message::IncrementPressed),
                )
                .push(
                    // We show the value of the counter here
                    Text::new(self.value.to_string()).size(50),
                )
                .push(
                    // The decrement button. We tell it to produce a
                    // `DecrementPressed` message when pressed
                    Button::new(&mut self.decrement_button, Text::new("-"))
                        .on_press(Message::DecrementPressed),
                )
        }
    }
    ~~~

    最後に、生成されたメッセージに反応し、更新ロジックでそれに応じて状態を変更できるようにする必要があります。

    ~~~rust
    impl Counter {
        // ...
    
        pub fn update(&mut self, message: Message) {
            match message {
                Message::IncrementPressed => {
                    self.value += 1;
                }
                Message::DecrementPressed => {
                    self.value -= 1;
                }
            }
        }
    }
    ~~~

    そして、それが全てだ！ユーザーインターフェース全体を書きました `Iced`ができるようになりました

    1. ビューロジックの結果を取得し、そのウィジェットをレイアウトします。
    2. システムからのイベントを処理し、更新ロジック用のメッセージを生成します。
    3. 結果のユーザーインターフェースを描画します。

    ---

    - Usage

      アプリケーションとサンドボックスの特性は、上記のすべてのプロセスを合理化して、すぐに始めることができるはずです!

    ---

    ### [Iced::Application](https://docs.rs/iced/0.2.0/iced/trait.Application.html)

    - Description

      インタラクティブなクロスプラットフォームのアプリケーション。

      この特徴は、Icedのメインの入り口です。一度実装されれば、単に run を呼び出すだけで GUI アプリケーションを実行することができます。

      - ネイティブプラットフォーム上では、それ自身のウィンドウで実行されます。
      - ウェブ上では、ドキュメントの <title> と <body> を制御します。

      アプリケーションはいくつかのメソッドでCommandを返すことで非同期アクションを実行することができます。プログラムの中でバックグラウンドでの作業を行うつもりがない場合、サンドボックスの特徴はシンプルなインターフェイスを提供します。デバッグ機能を有効にしたアプリケーションを使用している場合、F12キーを押すことでデバッグビューを切り替えることができます。

      ---

    - Example

      [リポジトリ](https://github.com/hecrj/iced/tree/0.2/examples)には、`Application trait`を使用したサンプル集があります。

      - clock: Canvasウィジェットを使って時計を描き、その針で現在の時刻を表示するアプリケーションです。
      - `download_progress`: 100 MB のダミーファイルを非同期でダウンロードし、ダウンロードの進行状況を追跡する基本的なアプリケーションです。
      - `events`: 条件付きサブスクリプションを使って表示されるネイティブイベントのログ。
      - `game_of_life`:  `[John Horton Conway]`が考案した`「Game of Life」`のインタラクティブ版。
      - `pokedex`: `PokéAPI`を使ってランダムにポケデックスのエントリー(スプライト付き！)を表示するアプリケーション。
      - `solar_system`: `Canvas`ウィジェットを使って描かれた太陽系のアニメーション。
      - `stopwatch`:  スタート/ストップとリセットボタンが付いた時計で、時間を聞く方法を紹介しています。
      - `todos`: `TodoMV`にインスパイアされた`TODOS`トラッカー。

    ---

    - A Simple "Hello, World"

      ~~~rust
      use iced::{executor, Application, Command, Element, Settings, Text};
      
      pub fn main() -> iced::Result {
          Hello::run(Settings::default())
      }
      
      struct Hello;
      
      impl Application for Hello {
          type Executor = executor::Default;
          type Message = ();
          type Flags = ();
      
          fn new(_flags: ()) -> (Hello, Command<Self::Message>) {
              (Hello, Command::none())
          }
      
          fn title(&self) -> String {
              String::from("A cool application")
          }
      
          fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
              Command::none()
          }
          
          fn view(&mut self) -> Element<Self::Message> {
              Text::new("Hello, world!").into()
          }
      }
      ~~~


    ---
    
    - Required Methods
    
      - `pub fn new(flags: Self::Flags) -> (Self, Command<Self::Message>)`
    
        設定の一部として実行([run](https://docs.rs/iced/0.2.0/iced/trait.Application.html#method.run))するために提供されたフラグで[Application](https://docs.rs/iced/0.2.0/iced/trait.Application.html)を初期化します。
    
        ここでアプリの初期状態を返します。
    
        さらに、起動時にバックグラウンドで非同期アクションを実行する必要がある場合は、[Command](https://docs.rs/iced/0.2.0/iced/struct.Command.html)を返すことができます。これは、ファイルから状態をロードしたり、最初のHTTPリクエストを実行したりする場合などに便利です。
    
      - `pub fn title(&self) -> String`
    
        アプリケーションの現在のタイトルを返します。
    
        このタイトルは動的なものにすることができます！ランタイムは必要に応じてアプリケーションのタイトルを自動的に更新します。
    
      - `pub fn update(&mut self, message: Self::Message) -> Command<Self::Message>`
    
        メッセージを処理し、[Application](https://docs.rs/iced/0.2.0/iced/trait.Application.html)の状態を更新します。
    
        ここで更新ロジックを定義します。ユーザーのインタラクションやコマンドによって生成されるすべてのメッセージは、このメソッドによって処理されます。
    
        返された[Command](https://docs.rs/iced/0.2.0/iced/struct.Command.html)は、バックグラウンドで直ちに実行されます。
    
      - `pub fn view(&mut self) -> Element<'_, Self::Message>`
    
        [Application](https://docs.rs/iced/0.2.0/iced/trait.Application.html)に表示するウィジェットを返します。
    
        これらのウィジェットは、ユーザーのインタラクションに基づいてメッセージを生成することができます。


​    

---

### [iced::HorizontalAlignment::Center](https://docs.rs/iced/0.2.0/iced/enum.HorizontalAlignment.html#variant.Center)

* Description

  水平方向の中央揃え

---

### [iced::widget::canvas::Text::horizontal_alignment](https://docs.rs/iced/0.2.0/iced/widget/canvas/struct.Text.html#structfield.horizontal_alignment)

* Description

  テキストの水平方向の配置

---

### [iced::Length::Fill](https://docs.rs/iced/0.2.0/iced/enum.Length.html#variant.Fill)

* Description

  残りのスペースをすべて埋める

---

### iced::Subscription::from_recipe

- Description

  サブスクリプションを記述するRecipeからサブスクリプションを作成します。

---

### [iced_futures](https://docs.rs/iced_futures/0.2.0/iced_futures/)

- Description

  Elmにインスパイアされた、GUIプログラミングのための非同期タスクです。

---

### [iced_native](https://docs.rs/iced_native/0.3.0/iced_native/)

- Description

  レンダラに依存しないネイティブGUIランタイム。

  Iced エコシステムのネイティブパス

  `iced_native`は、`iced_core`の上にネイティブランタイムを構築するもので、以下の特徴があります。

  - `druid`にインスパイアされたカスタムレイアウトエンジン
  - すべての組み込みウィジェットのイベント処理
  - レンダラーに依存しないAPI

  これを実現するために、再利用可能なインターフェースをいくつか導入しています。

  - `Widget trait`は、レイアウト要件からイベントや描画のロジックまで、新しいウィジェットを実装するために使用されます。
  - `Renderer trait`の束で、これは、クレートを`Renderer`に依存しないようにするためのものです。

---

### [iced_native::subscription::Recipe](https://docs.rs/iced_native/0.3.0/iced_native/subscription/trait.Recipe.html)

- Description

  [Subscription](https://docs.rs/iced_futures/0.2.0/iced_futures/subscription/struct.Subscription.html)の説明です。

  [Recipe](https://docs.rs/iced_native/0.3.0/iced_native/subscription/trait.Recipe.html)は、[Subscription](https://docs.rs/iced_futures/0.2.0/iced_futures/subscription/struct.Subscription.html)の内部定義です。それは、サブスクリプションを実行して識別するためにランタイムによって使用されます。これを使って自分のものを作ることができます

- Example

  リポジトリには、カスタム[Recipe](https://docs.rs/iced_native/0.3.0/iced_native/subscription/trait.Recipe.html)を使った[サンプル](https://github.com/hecrj/iced/tree/0.2/examples)がいくつかあります。

  [download_progress](https://github.com/hecrj/iced/tree/0.2/examples/download_progress): 100MBのダミーファイルを非同期にダウンロードし、ダウンロードの進行状況を追跡する基本的なアプリケーションです。
  [stopwatch](https://github.com/hecrj/iced/tree/0.2/examples/stopwatch): スタート/ストップとリセットボタンを備えた時計で、時間の聴き方を紹介しています。



