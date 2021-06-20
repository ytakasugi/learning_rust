use proconio::input;

fn main() {
    input! {
        mut a: [i32; 3],
    }

    // 要素をソートする
    a.sort();

    // `a[2] - a[1]`と`a[1] - a[0]`を計算する  
    let seq1 = a[2] - a[1];
    let seq2 = a[1] - a[0];

    // `a[2] - a[1] == a[1] - a[0]`が成立するなら`Yes`を出力する
    if seq1 == seq2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
