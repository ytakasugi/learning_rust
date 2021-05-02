fn main() {
    let c1 = 'A';         // char型
    println!("c1={}", c1);
    let c1_ptr = &c1;     // &char型。不変の参照（イミュータブルな参照）
    println!("c1_ptr={}", c1_ptr);
    assert_eq!(*c1_ptr, 'A');

    let mut n1 = 0;       // i32型
    println!("n1={}", n1);
    let n1_ptr = &mut n1; // &mut i32型。可変の参照（ミュータブルな参照）
    println!("n1_ptr={}", n1_ptr);
    assert_eq!(*n1_ptr, 0);

    // 可変の参照では参照先の値を変更できる
    *n1_ptr = 1_000;
    println!("*n1_ptr={}", *n1_ptr);
    assert_eq!(*n1_ptr, 1_000);
}