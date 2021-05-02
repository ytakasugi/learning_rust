// `Fn`トレイトを境界とするジェネリックな引数`F`を定義し、それを呼び出す。
fn call_me<F: Fn()>(f: F) {
    f();
}

// `Fn`トレイト境界を満たすラッパー関数を定義
fn function() {
    println!("I'm a function!");
}

fn main() {
    let closure = || println!("I'm a closure");

    call_me(closure);
    call_me(function);
}
