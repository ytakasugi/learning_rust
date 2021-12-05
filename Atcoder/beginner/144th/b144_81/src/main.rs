use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let mut sum = 0;

    for i in 1..10 {
        for j in 1..10  {
            if i * j == n {
                sum += 1
            }
        }
    }

    if sum >= 1 {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
