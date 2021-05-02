// この関数は、`Box<i32>`の所有権を奪い、破棄する
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// この関数は`i32`を借用する
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    // Box化された`i32`型を作成
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Boxの中身を借用する。所有権を奪うわけではないため、直後に使用できる
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Box内の要素に対する参照
        let ref_to_i32: &i32 = &boxed_i32;
        println!("ref_to_i32: {}", ref_to_i32);

        //スコープ内で内部の値を後で借りている間は`boxed_i32`を破棄できない。
        // eat_box_i32(boxed_i32);

        // `eat_box_i32`で値が破棄されているため、エラーとなる
        borrow_i32(ref_to_i32);
    }
    // ここでようやく、`eat_box`は所有権を移譲し、破棄することができます。
    eat_box_i32(boxed_i32);
}
