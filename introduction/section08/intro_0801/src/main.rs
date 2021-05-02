use std::io;

fn main() {
    println!("Please enter a n1:");
    let n1 = get_input();

    // 返り値の型はいずれのブロックでも同一でなければならない。
    if n1 < 0 {
        println!("{} is negative", n1);
    } else if n1 > 0 {
        println!("{} is positive", n1);
    } else {
        println!("{} is zero", n1)
    }

    println!("Please enter a n2:");
    let n2 = get_input();

    if n2 < 10 && n2 > -10 {
        println!(", and is a small number, increase ten-fold");
        n2 * 10
    } else {
        println!(", and is a big number, halve the number");
        n2 / 2
    };

    println!("{} -> {}", n1, n2);

    // if文は式なので、let文の右辺で使用できる
    let condition = true;
    let n3 = if condition {
        5
    } else {
        6
    };

    println!("The value of n3 is: {}", n3)

}

// コンソールから入力したString型を数値型に変換する関数
fn get_input() -> i32 {
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok();
    return n.trim().parse().ok().unwrap();
}
