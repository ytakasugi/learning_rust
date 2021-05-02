struct Owner(i32);

// `Owner`構造体に`add_one`メソッドと`print`メソッドを定義
impl Owner {
    // 通常の関数と同様にライフタイムを明示
    // 引数に`&mut self`としているので、`Owner::add_one`メソッド内で`Owner`構造体(のインスタンス)を`self`という名前の可変参照として扱える
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
