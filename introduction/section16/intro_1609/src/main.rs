trait UsernameWidget {
    // このウィジェットから選択されたユーザ名を取得する
    fn get(&self) -> String;
}

trait AgeWidget {
    // このウィジェットの選択された年齢を取得するこのウィジェットの選択された年齢を取得する
    fn get(&self) -> u8;
}

// UsernameWidget と AgeWidget の両方を持つフォーム
struct Form {
    username: String,
    age: u8,
}

// `Form`メソッドに`UsernameWidget`トレイトを実装
impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

// `Form`メソッドに`AgeWidget`トレイトを実装
impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 25,
    };

    // If you uncomment this line, you'll get an error saying 
    // "multiple `get` found". Because, after all, there are multiple methods
    // named `get`.
    // println!("{}", form.get());

    // 完全修飾構文を使用し、どのメソッド(`get`)を使用するか明示的に示す必要がある
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(25, age);


}
