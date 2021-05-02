use proconio::input;

fn main() {
    input! {
        a: String,
    }

    let v = a.chars().collect::<Vec<char>>();
    println!("{}{}{}", v[1], v[2], v[0]);

}