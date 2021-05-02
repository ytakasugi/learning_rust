use proconio::input;

fn main() {
    input!{
        s: i32,
        w: i32,
    }

    // 狼の数が羊の数以上のとき
    if s <= w {
        println!("unsafe");
    // 狼の数が羊の数未満のとき
    } else if s > w {
        println!("safe");
    }
}
