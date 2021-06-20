use proconio::input;

fn main() {
    input!{
        n: usize,
        mut v: [(String, u64); n]
    }

    v.sort_by_key(|k| k.1);

    println!(
        "{}",
        // ソートしてあるため、後ろから2番目の要素が解となる
        v[n - 2].0
    );
}
