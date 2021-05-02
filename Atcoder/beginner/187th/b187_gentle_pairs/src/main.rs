use proconio::input;

fn main() {
    input! {
        p: [(i32, i32)],
    }

    let mut ans = 0;
    // p.len() はベクタ p の要素数
    for i in 0..p.len() {
        for j in 0..i {
            // 点 i と点 j が条件を満たすか調べる
            let (dx, dy) = {
                let (x1, y1) = p[j];
                let (x2, y2) = p[i];
                ((x1 - x2).abs(), (y1 - y2).abs())
            };
            // dx は 2 点の x 座標の差の絶対値
            // dy は 2 点の y 座標の差の絶対値
            if dx >= dy {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
