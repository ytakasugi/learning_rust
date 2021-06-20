use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
        b: [i32; n],
    }

    let a_max = a.iter().max();
    let b_min = b.iter().min();

    let result = 0.max(b_min.unwrap() - a_max.unwrap() + 1);
    println!("{}", result);
}
