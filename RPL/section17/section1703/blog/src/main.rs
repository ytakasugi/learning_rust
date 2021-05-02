extern crate blog;
use blog::Post;

fn main() {
    // `Post::new`で空の文字列を生成
    let mut post = Post::new();

    // `add_text`メソッドを`post`に対して呼び出し、文字列を挿入
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}