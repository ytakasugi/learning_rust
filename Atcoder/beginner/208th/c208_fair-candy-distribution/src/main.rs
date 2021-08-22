use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
        mut a: [i32; n],
    }

    // a を (index, 番号) の形にしてから番号でソートする
    // 型がかわるので shadowing をする
    let mut a = a.iter().enumerate().collect::<Vec<_>>();
    a.sort_by_key(|&(_, a)| a);

    // n 人に k / n 個ずつ配る
    // 残りキャンディが k % n 個になる
    let mut ans = vec![k / n; n as usize];

    // `a`が番号順に並んでいるので、番号の小さいほうからk % n人にキャンディを配る
    for i in 0..(k % n) as usize {
        let idx = a[i].0;
        ans[idx] += 1;
    }

    for i in 0..n as usize {
        println!("{}", ans[i]);
    }
}
