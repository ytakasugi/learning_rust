// タプル構造体`Container`を定義
struct Container(i32, i32);

// `A`と`B`は`type`キーワードを用いてトレイト内で宣言されている。
// (注意: この文脈で使用する`type`は型エイリアスを宣言する際の`type`とは異なることに注意)
// ２つの要素がコンテナ型の中にあることをチェックするトレイト
// また、最初と最後の値を取得することもできる
trait Contains {
    // メソッドが使用できるジェネリック型を定義
    type A;
    type B;

    // `A`と`B`の両方を明示的に要求する
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    // `A`、`B`いずれも要求しない
    fn first(&self) -> i32;
    // `A`、`B`いずれも要求しない
    fn last(&self) -> i32;
}

impl Contains for Container {
    // `A`と`B`がどの型であるかを明示。インプットの型（訳注: つまり`Self`の型）が`Container(i32, i32)`である場合、
    // 出力型は`i32`と`i32`となる。
    type A = i32;
    type B = i32;

    // コンテナ内の２つの要素が等しければTrueを返す
    // `&i32`の代わりに`&Self::A`または`&self::B`と書いても良い
    fn contains(&self, n: &Self::A, m: &Self::B) -> bool {
        (&self.0 == n) && (&self.1 == m)
    }

    // ひとつ目の値を取得
    fn first(&self) -> i32 {
        self.0
    }

    // 最後(2つ目)の値を取得
    fn last(&self) -> i32 {
        self.1
    }
}

// `A`と`B`は`C`に保持されていることを考慮すると、`A`と`B`を２度も書くのは面倒
fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let n = 3;
    let m = 10;
    let container = Container ( n, m);

    println!(
        "Does container contain {} and {}: {}"
        , n
        , m
        , container.contains(&n, &m)
    );

    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}