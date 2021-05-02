use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    // 選手の人数の半分を half とする
    let half = {
        // 1 に 2 を n - 1 回かけて
        // 2 の n - 1 乗を計算する
        let mut x = 1;

        // for 式の中で _i 自体は使わない．
        // つまり _i は「代入しただけで使わない変数」なので
        // 先頭にアンダースコアを付ける（ 05 章参照）
        for _i in 0..n - 1 {
            x *= 2;
        }
        x
    };
    input! {

        // 前半
        former: [i32; half],

        // 後半
        latter: [i32; half],
    }

    // 前半のうち，レートが最大の選手の番号
    let fmax = {
        let mut index = 0;
        for i in 1..half {
            if former[index] < former[i] {
                index = i;
            }
        }
        index
    };

    // 後半の中で，レートが最大の選手の番号
    let lmax = {
        let mut index = 0;
        for i in 1..half {
            if latter[index] < latter[i] {
                index = i;
            }
        }
        index
    };

    // 最後に 1 を足すのを忘れないこと
    let ans = if former[fmax] < latter[lmax] {
        fmax
    } else {
        half + lmax
    } + 1;
    println!("{}", ans);
}