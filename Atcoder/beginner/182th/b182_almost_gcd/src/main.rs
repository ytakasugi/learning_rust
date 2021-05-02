use proconio::input;

fn main() {
    input! {
        v: [i32],
    }

    // GCD 度の最大値
    let mut max = 0;
    // GCD 度を最大化する整数
    let mut res = -1;

    // A_1 〜 A_N の最大値が 1000 なので
    // 2 以上 1000 以下の範囲を調べればよい
    for i in 2..=1000 {
        // GCD 度を計算する
        let mut gcd_ness = 0;
        for a in &v {
            if a % i == 0 {
                gcd_ness += 1;
            }
        }

        // GCD 度の最大値と，
        // そのときの i の値を記録する
        if max < gcd_ness {
            max = gcd_ness;
            res = i;
        }
    }
    println!("{}", res)
}
