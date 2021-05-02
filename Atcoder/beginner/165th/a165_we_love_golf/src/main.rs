use proconio::input;

fn main() {
    input!{
        k: i32,
        n: i32,
        m: i32,
    }

    // m以下の最大のkの倍数を求め、それがn以上か確かめる
    let largest = (m / k) * k;
    if largest >= n {
        println!("OK");
    } else {
        println!("NG");
    }
}
