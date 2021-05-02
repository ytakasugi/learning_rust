use proconio::input;

fn main() {
    input! {
        // バッテリーの容量
        n: i32,
        m: usize,
        t: i32,
        mut cafe: [(i32, i32); m],
    }

    cafe.push((t, t));

    // バッテリーの残量
    let mut battery = n;

    // 最後にカフェを出た時刻
    let mut prev = 0;

    for &(a, b) in &cafe {
        // カフェに着くまでの間に減ったバッテリー残量
        battery -= a - prev;

        // 一度でも 0 以下になったら終わり
        if battery <= 0 {
            break;
        }

        // バッテリー残量は滞在時間（ b - a ）だけ増える
        // ただし n は超えない
        battery = n.min(battery + b - a);

        prev = b;

    }

    // 途中で一度でも 0 以下になると break されるので，
    // ここで残量が正であれば途中ずっと正だったということ
    if battery > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
