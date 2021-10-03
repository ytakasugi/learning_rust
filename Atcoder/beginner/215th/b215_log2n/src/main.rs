use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut k = 0;

    for i in 1.. {
        if 2_i64.pow(i) <= n {
            k = i;
        } else {
            break;
        }
    }
    println!("{}", k);
}
