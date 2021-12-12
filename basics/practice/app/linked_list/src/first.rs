// 再帰的な構造は、`Box`でラップする必要がある
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

