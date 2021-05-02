// データのミュータビリティは所有権を移譲した際に変更できます。

fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // `immutable_box`の所有権を`mutable_box`にムーブし、同時にミュータビリティを変更する
    let mut mutable_box = immutable_box;
    
    println!("mutable_box contains {}", mutable_box);

    // `mutable_box`の内容を変更する
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
