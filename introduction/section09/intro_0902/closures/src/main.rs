fn main() {
    // 値をインクリメントする関数
    fn function(i: i32) -> i32 {
        i + 1
    }

    // 型アノテーションと`{}`は。関数と同様の方法で行えるが必須でない
    let closure_annotated = |i: i32| -> i32 {
        i + 1
    };

    let closure_inferred = |i| i + 1;

    let i = 1;

    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 引数を取らずに`i32`を返すクロージャ
    // 引数は推論される
    let one = || 1;
    println!("closure returning one: {}", one());

}
