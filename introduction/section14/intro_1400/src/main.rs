// 具象型Aを定義
struct A;

// 具象型Singleを定義
struct Single(A);

// ジェネリックな具象型SingleGenを定義
struct SingleGen<T>(T);

fn main() {
    // `Single`は具象型で、`A`のみを受け取る。
    let _s = Single(A);

    // `_char`という名の変数を生成する。これは`SingleGen<char>`という型で、
    // 値は`SingleGen('a')`となる。ここでは、`SingleGen`には明示的な型パラメータが与えられている。
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen`型の変数には明示的に型パラメータを与えなくてもよい。
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
}
