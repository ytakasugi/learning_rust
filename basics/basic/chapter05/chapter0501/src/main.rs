fn main() {
    // (i32, String)型のタプル。スタックに置かれる
    let t1 = (5, "birds".to_string());
    // Boxポインタを作る。タプルがヒープに移動する
    let mut b1 = Box::new(t1);
    // *で参照外し
    (*b1).0 += 1;
    assert_eq!(*b1, (6, "birds".to_string()));
}
