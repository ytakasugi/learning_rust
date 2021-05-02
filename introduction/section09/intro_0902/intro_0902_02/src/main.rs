use std::mem;

// // クロージャを引数に取り、それを呼び出す関数。
// <F>はFが "汎用型パラメータ "であることを示します。
fn apply<F>(f: F)
where
    // クロージャには引数も返り値もない。
    // `F`は`FnOnce`を実装し、引数も返り値もない
    F: FnOnce() {
        f();
    }

// クロージャを引数に取り、`i32`を返す関数
fn apply_to_3<F>(f: F) -> i32
where
    // このクロージャは引数、返り値ともに`i32`
    // `F`は`Fn`を実装し、`i32`を引数にとり`i32`を返す
    F: Fn(i32) -> i32 {
        f(3)
    }

fn main() {
    let greeting = "hello";

    // コピーではなくmoveが起きる型
    let mut farewell = "goodbye".to_owned();

    // 変数を2つ補足。`greeting`は参照を、
    // `farewell`は値をそれぞれ捕捉する。
    let diary = || {
        // `greeting`は参照なので、`Fn`が必要。
        println!("I said {}", greeting);

        // `farewell`の値を変更する(可変参照)ので、この時点で`FnMut`が必要になる。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // `mem::drop`を明示的に呼ぶと`farewell`が値で捕捉される必要性が発生する。
        // よって`FnOnce`が必要になる。
        mem::drop(farewell);
    };

    // クロージャを適用する関数を実行。
    apply(diary);

    // double` は `apply_to_3` の特徴量を満たす。
    let d = |x| 2 * x;
    let i = |y| 3 * y;

    println!("d: {}", apply_to_3(d));
    println!("i: {}", apply_to_3(i));
}
