use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let mut ans=0;
    let mut t = 0;
    for i in 1.. {
        t += i;
        if t >= n {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}
