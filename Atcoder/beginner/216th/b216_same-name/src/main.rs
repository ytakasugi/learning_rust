use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [(String, String); n],
    }

    for i in 0..n {
        for j in i + 1..n {
            if v[i].0 == v[j].0 && v[i].1 == v[j].1 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
