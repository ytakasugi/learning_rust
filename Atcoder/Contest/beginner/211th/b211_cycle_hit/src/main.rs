use proconio::input;

fn main() {
    input! {
        mut s: [String; 4],
    }

    s.sort();
    s.dedup();

    if s.len() == 4 {
        println!("Yes");
    } else {
        println!("No");
    }
}
