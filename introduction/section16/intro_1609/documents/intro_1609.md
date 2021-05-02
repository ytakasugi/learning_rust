### [Disambiguating overlapping traits](https://doc.rust-jp.rs/rust-by-example-ja/trait/disambiguating.html#disambiguating-overlapping-traits)

1つの型には多くの異なるトレイトを実装することができます。2つのトレイトが両方とも同じ名前を必要とする場合はどうすればよいでしょうか？例えば、多くのトレイトは`get()`という名前のメソッドを持っているかもしれません。また、異なる戻り値の型を持つこともあるかもしれません。

良いニュースです: それぞれの `trait`の実装には独自の`impl`ブロックがあるので、どの`trait`の`get`メソッドを実装しているのかが明確になります。

では、これらのメソッドを呼び出すときはどうすればいいのでしょうか？これらのメソッドを区別するために、完全修飾構文を使用しなければなりません。

~~~rust
trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the selected age out of this widget
    fn get(&self) -> u8;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form{
        username: "rustacean".to_owned(),
        age: 28,
    };

    // If you uncomment this line, you'll get an error saying 
    // "multiple `get` found". Because, after all, there are multiple methods
    // named `get`.
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}
~~~

- See also

  [Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name](https://doc.rust-jp.rs/book-ja/ch19-03-advanced-traits.html#%E6%98%8E%E7%A2%BA%E5%8C%96%E3%81%AE%E3%81%9F%E3%82%81%E3%81%AE%E3%83%95%E3%83%AB%E3%83%91%E3%82%B9%E8%A8%98%E6%B3%95-%E5%90%8C%E3%81%98%E5%90%8D%E5%89%8D%E3%81%AE%E3%83%A1%E3%82%BD%E3%83%83%E3%83%89%E3%82%92%E5%91%BC%E3%81%B6)
