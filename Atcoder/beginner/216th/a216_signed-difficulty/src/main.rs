use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let xy = s.split(".").collect::<Vec<_>>();
    let y: usize = xy[1].parse().unwrap();

    if y <= 2 {
        println!("{}-", xy[0]);
    } else if y <= 6 {
        println!("{}", xy[0]);
    } else {
        println!("{}+", xy[0]);
    }
}
