use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut v = s.chars().collect::<Vec<char>>();
    v.remove(3);
    v.insert(3, '8');

    for i in v.iter() {
        print!("{}", i);
    }
}
