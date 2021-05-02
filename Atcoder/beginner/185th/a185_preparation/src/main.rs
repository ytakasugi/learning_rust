use proconio::input;

fn main() {
    input! {
        a: [i32; 4],
    }

    let mut ans = 100;

    for &i in &a {
        if ans > i {
            ans = i;
        }
    }

    println!("{}", ans);
}
