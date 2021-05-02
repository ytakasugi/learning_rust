#![allow(unused)]
pub struct Post {
    // `Option`で`Box<State>`のトレイトオブジェクトを保持する
    //state: Option<Box<dyn State>>,
    content: String,
}

pub struct DraftPost {
    content: String,
}

// `Post`構造体にメソッドを追加する
impl Post {
    /*
    pub fn new() -> Post {
        Post {
            // 新しいPostを作る時、`state`フィールドは、`Box`を保持する`Some`値にセットします。
            // この`Box`が`Draft`構造体の新しいインスタンスを指します。
            // これにより、 新しい`Post`を作る度に、草稿から始まることが保証されます。
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // `self`への可変参照をとることで、`add_text`を呼び出した`Post`インスタンスを変更する
    pub fn add_text(&mut self, text: &str) {
        // `content`に対して`push_str`を呼び出して、text`引数を渡して保存されたcontentに追加する
        self.content.push_str(text);
    }
    */

    // `Post`のインスタンスを返すのではなく、`DraftPost`のインスタンスを返す
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }


    /*
    // 空の文字列スライスを返すメソッド
    //  状態がPublishedなら、 記事のcontentフィールドの値を返す
    // それ以外なら、空の文字列スライスを返す。
    pub fn content(&self) -> &str {
        // `self(=&Post)の`state`値に対して`as_ref`メソッドを呼び出し、`Option<Box<dyn State>>`から`Option<&Box<State>>`に変換
        // そして、`unwrap`で`Some`値を取り出す。
        // `&Box<State>`に対して、`content`メソッドを呼び出し、参照外し型型強制が参照(`&`)と`Box`に働くので、究極的に`content`メソッドが`State`トレイトを実装する型に対して呼び出されることになる
        self.state.as_ref().unwrap().content(&self)
    }
    */

    pub fn content(&self) -> &str {
        &self.content
    }

    /*
    pub fn request_review(&mut self) {
        // `take`メソッドを呼び出して、`Some`値を取り出し、`None`を残す。
        // これにより、`Post`の`state`値をムーブすることができる
        if let Some(s) = self.state.take() {
            // 直接`state`値の所有権を得るよう設定するのではなく、 一時的に`None`に`state`をセットする必要があります。
            // これにより、新しい状態に変形した後に、 `Post`が古い`state`値を使えないことが保証される
            self.state = Some(s.request_review())
        }
    }
    */

    /*
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    */
}

// `State`トレイトは、異なる記事の状態で共有される振る舞いを定義し、
// Draft、PendingReview、Published状態は全て、 Stateトレイトを実装する
trait State {
    // `self: Box<Self>`という記法は、型を保持する`Box`に対して呼ばれたときのみ、このメソッドが合法となることを意味している。
    // この記法は、`Box<Self>`の所有権を奪い、古い状態を無効化するので、`Post`の状態値は新しい状態に変形できる。
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 空の文字列スライスを返すデフォルト実装を`content`メソッドに追加
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

// `content`メソッドが実装されていないことにより、すべての記事が草稿記事から始まり、草稿記事は表示できる内容がないことを保証している
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // `DraftPost`に`PendingReviewPost`を返す`request_review`メソッドを定義
    // `self`の所有権を奪う、すなわち`DraftPost`インスタンスを消費し、`PendingReviewPost`に変換する
    // `PendingReviewPost`を得る唯一の方法は、 `DraftPost`に`request_review`を呼び出すこと
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

struct Draft {}

// `Draft`構造体に`State`トレイトを実装する
impl State for Draft {
    // 新しい`PendingReview`構造体の新しいボックスのインスタンスを返す必要があり、 これが、記事が査読待ちの時の状態を表す。
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // `Post`を返す`approve`メソッドを定義
    // `self`の所有権を奪う、すなわち`DraftPost`インスタンスを消費し、`Post`に変換する
    // `Post`インスタンスを得る唯一の方法は、 `PendingReviewPost`に対して`approve`を呼び出すこと
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

// `PendingReview`構造体に`State`トレイトを実装する
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // `approve`メソッドを呼び出すと、`Published`構造体の新しい`Box`化されたインスタンスを返す
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // `content`メソッドをオーバーライド
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
