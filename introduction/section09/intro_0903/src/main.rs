// 高階関数:関数を引数にとるか、関数を戻り値として返す関数
fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    // 1000以下の奇数を2乗した値の合計を求める。
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // 宣言型プログラミングによるアプローチ
    // 値を蓄積する変数を宣言
    let mut acc = 0;

    // 0から無限までイテレートする
    for n in 0.. {
        // 値を2乗
        let squared = n * n;

        if squared >= upper {
            // 上限に達した場合、ループを終了
            break;
        } else if is_odd(squared) {
            // 奇数ならば値を足していく
            acc += squared
        }
    }
    println!("imperative style: {}", acc);

    // 関数型プログラミングによるアプローチ
    let sum_of_squared_odd_numbers: u32 =
        // 全自然数を2乗
        (0..).map(|n| n * n)
        // そのうち上限より小さい値
        .take_while(|&squared| squared < upper)
        // かつ奇数のもの
        .filter(|&squared| is_odd(squared))
        // 足し合わせる。
        .fold(0, |acc, squared| acc + squared);

        println!("functional style: {}", sum_of_squared_odd_numbers);
}
