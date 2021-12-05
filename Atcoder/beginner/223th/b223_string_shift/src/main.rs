use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut v = Vec::new();

    for i in 0..s.len() {
        v.push(s[i..].to_string() + &s[0..i]);
    }
    v.sort();
    println!("{}\n{}", v[0], v[s.len() - 1]);
}
