// クロージャを返す関数
fn return_closure(init: i32) -> impl FnMut() -> i32 {
    let mut x = init;
    move || {
        let ret = x;
        x += 1;
        ret
    }
}

// クロージャを返す関数を使用するには、`function()` + クロージャに与える引数(`()`)としなければならない。
// 今回であれば、`FnMut`は引数を取らないのでユニットとなる
fn main() {
    println!("{}", return_closure(1)());
}