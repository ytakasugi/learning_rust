use iced::{
    button, executor, Align, Application, Button, Column, Command, Element, Font,
    HorizontalAlignment, Length, Row, Settings, Subscription, Text,
};
use iced_futures::{self, futures};
use std::time::{Duration, Instant};

fn main() {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 400u32);
    GUI::run(settings);
}

// `include_bytes`マクロは、指定したファイルの内容を実行バイナリに添付します。
//  これをバイナリ配列として使えばいいのです。
const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

// グローバル変数を定義
const FPS: u64 = 30;
const MILLISEC: u64 = 1000;
const MINUTE: u64 = 60;
const HOUR: u64 = 60 * MINUTE;

#[derive(Debug, Clone)]
pub enum Message {
    // 時間の測定を開始するメッセージ
    Start,
    // 時間の測定を停止するメッセージ
    Stop,
    // 測定した時間をリセットするメッセージ
    Reset,
    Update,
}

// アプリケーションが測定中かどうか管理するための列挙型
pub enum TickState {
    Stopped,
    Ticking,
}

// 構造体`GUI`を定義
struct GUI {
    last_update: Instant,
    total_duration: Duration,
    tick_state: TickState,
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

// icedクレートを使用する場合は、ApplicationかSandboxのどちらかを選択する必要があります。
//
// アプリケーションはサンドボックスよりも多くの機能を持っています。
// 先進的なアプリケーションの機能の一つにSubscriptionがあります。
// 非同期的な機能を実行することができます。
//
// ゲームやアニメのように一定間隔でウィンドウの内容を再描画したい場合は、サブスクリプションを利用してオリジナルの再描画イベントを作成する必要があります。
//
// そして、初期フラグとオプションを扱う必要がある場合は、アプリケーションを選択する必要があります。
//
// リッチなUIを必要とせず、非同期実行のないシンプルなUIが必要なだけなら、Sandboxを選ぶべきです。
//
// 今回の場合は、一定間隔でウィンドウの内容を再描画したいので、Applicationを使用します。

// GUI構造体に`Application`を実装する
impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    // `new()`メソッドは、`Application`トレイトを`run()`した際に、`iced`の内部で使用される初期化のために使用されるメソッド。
    // 自分で`new()`を実行する必要はない
    // 引数からフラグを得ることができるので、そのフラグに従って初期化処理を分岐することができる
    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI{
                last_update: Instant::now(),
                total_duration: Duration::default(),
                tick_state: TickState::Stopped,
                // `start_stop_button_state`と`reset_botton_state`を初期化
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new()
            }, 
            Command::none(),
        )
    }

    // `FPS`は、`Frame Per Second`の略
    // `MILLISEC`を`FPS`で割った時間だけ間をあけてイベントを受信する
    fn subscription(&self) -> Subscription<Message> {
        let timer = Timer::new(Duration::from_millis(MILLISEC / FPS));
        // `iced::Subscription::from_recipe`でRecipeをSubscriptionとして動作させる。
        // ここでは、`map()`を使用して、それぞれの要素を`Message::Update`に置き換えている
        iced::Subscription::from_recipe(timer).map(|_| Message::Update)
    }

    // ウィンドウのタイトルを設定します。
    fn title(&self) -> String {
        String::from("Timer")
    }

    // このメソッドは Message をイベントとして受け取ります。
    // また、メッセージによってアプリケーションの状態を変更します。
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            // 状態変数を`TickState::Ticking`に切り替え、最終更新時刻を保持している`last_update`に現在の時刻を保存する
            Message::Start => {
                self.tick_state = TickState::Ticking;
                self.last_update = Instant::now();
            }
            // 状態変数を`TickState::Stopped`に切り替え、時間の最終更新から現在までの経過時間を`total_duration`に加算する
            Message::Stop => {
                self.tick_state = TickState::Stopped;
                self.total_duration += Instant::now() - self.last_update;
            }
            // 最終更新時刻を現在の時刻にし、これまで加算してきた経過時間をゼロにする
            Message::Reset => {
                self.last_update = Instant::now();
                self.total_duration = Duration::default();
            }
            // 最終更新時間から、経過時間を`total_duration`に加算する。
            // 加算したあとは、最終更新時間を現在の時間にする
            Message::Update => match self.tick_state {
                TickState::Ticking => {
                    let now_update = Instant::now();
                    self.total_duration += now_update - self.last_update;
                    self.last_update = now_update;
                }
                _ => {}
            },
        }
        // 非同期処理の実行をランタイム側へ依頼することができる仕組み
        // 初期化時点で行いたい別処理を`Command`を使用して非同期で実行できる
        Command::none()
    }

    // このメソッドは、Windowに表示するウィジェットを返します。
    fn view(&mut self) -> Element<Self::Message> {
        let seconds = self.total_duration.as_secs();
        let duration_text = format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.total_duration.subsec_millis() / 10,
        );

        // `start/stop`テキストを準備
        let start_stop_text = match self.tick_state {
            // 測定中でなければ、`Start`ボタン
            TickState::Stopped => Text::new("Start")
                // テキストの水平方向の配置を中央寄せにする
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
            // 測定中であれば、`Stop`ボタン
            TickState::Ticking => Text::new("Stop")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        };

        // `start/stop`ボタンを押下したときの、メッセージを準備
        let start_stop_message = match self.tick_state {
            // 計測中出ない場合、startのメッセージ
            TickState::Stopped => Message::Start,
            // 計測中の場合、stopのメッセージ
            TickState::Ticking => Message::Stop,
        };

        // ウィジェットの初期化
        let tick_text = Text::new(duration_text).font(FONT).size(60);
        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            start_stop_text
        )
        .min_width(80)
        // ボタンが押されたときに表示されるメッセージを設定
        .on_press(start_stop_message);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80)
        // ボタンが押されたときに表示されるメッセージを設定
        .on_press(Message::Reset);

        Column::new()
            // 行に要素を追加
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    // 要素間の垂直方向の間隔を設定
                    .spacing(10)
            )
            // 要素間の垂直方向の間隔を設定
            .spacing(10)
            // 列のパディングを設定
            .padding(10)
            // カラムの幅を設定
            .width(Length::Fill)
            // カラムの高さを設定
            .height(Length::Fill)
            // カラムの内容の水平方向のアライメントを設定
            .align_items(Align::Center)
            .into()
    }
}

// このサブスクリプションのレシピは、指定された間隔で繰り返しイベントを生成します。
//間隔は、メンバー変数 "duration" で決まります。
pub struct Timer {
    duration: Duration,
}

impl Timer {
    fn new(duration: Duration) -> Timer {
        Timer{ duration: duration }
    }
}

// これは、サブスクリプションのレシピです。
impl<H, E> iced_native::subscription::Recipe<H, E> for Timer
where
    H: std::hash::Hasher,
{
    // サブスクリプションはこの`Output Type`を生成します。
    type Output =Instant;

    // このレシピをハッシュします。
    // このハッシュを使って各サブスクリプションを識別します。
    // 任意のメンバー変数からハッシュを生成できます。
    fn hash(&self, state: &mut H) {
        use std::hash::Hash;
        std::any::TypeId::of::<Self>().hash(state);
        self.duration.hash(state);
    }

    // このメソッドは、このレシピの実行と、そのサブスクリプションのイベント・ストリームの生成を担当します。
    // ストリームは、定義した Output タイプを出力します。
    // この例では、`Output`タイプは`Instant`です。
    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, E>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        // `async_std::stream::interval`を使用して、一定間隔で現在の時間を返す`Stream`を作成する
        async_std::stream::interval(self.duration)
            .map(|_| Instant::now())
            .boxed()
    }
}