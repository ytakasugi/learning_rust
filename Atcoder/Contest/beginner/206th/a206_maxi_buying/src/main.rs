use proconio::input;

fn main() {
    input!{
        n: f32,
    }

    let ans = (1.08 * n).floor();

    if ans == 206. {
        println!("so-so");
    } else if ans < 206. {
        println!("Yay!");
    } else {
        println!(":(");
    }
}
