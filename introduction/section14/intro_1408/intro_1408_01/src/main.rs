// タプル構造体`Container`を定義
struct Container(i32, i32);

// ２つの要素がコンテナ型の中にあることをチェックするトレイト
// また、最初と最後の値を取得することもできる
trait Contains<A, B> {
    // `A`と`B`の両方を明示的に要求する
    fn contains(&self, _: &A, _: &B) -> bool;
    // `A`、`B`いずれも要求しない
    fn first(&self) -> i32;
    // `A`、`B`いずれも要求しない
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    // コンテナ内の２つの要素が等しければTrueを返す
    fn contains(&self, n: &i32, m: &i32) -> bool {
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
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>
{
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