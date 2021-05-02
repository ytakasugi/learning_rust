fn main() {
    let c1 = 'A';                   // char型
    println!("c1={}", c1);
    // `&`で参照を作り、型強制で生ポインタに変換する
    let c1_ptr: *const char = &c1;  // *const char型。不変の生ポインタ
    println!("&c1={}", &c1);
    // 生ポインタの参照外しはunsafeな操作
    assert_eq!(unsafe { *c1_ptr }, 'A');

    let mut n1 = 0;                 // i32型
    println!("n1={}", n1);
    let n1_ptr: *mut i32 = &mut n1; // *mut i32型。可変の生ポインタ
    println!("&mut n1={}", &mut n1);
    assert_eq!(unsafe { *n1_ptr }, 0);

    // 可変の生ポインタでは参照先の値を変更できる
    unsafe {
        *n1_ptr = 1_000;
        println!("*n1_ptr={}", *n1_ptr);
        assert_eq!(*n1_ptr, 1_000);
    }
}