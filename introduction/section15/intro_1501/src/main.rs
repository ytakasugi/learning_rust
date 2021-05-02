fn create_box() {
    // 整数をヒープ上に確保
    let _box1 = Box::new(3i32);
    // println!("{:?}", box1);
    // `box1`はここでスコープを抜けるため破棄されて、メモリは解放される
}

#[derive(Debug)]
struct ToDrop;

// `ToDrop`に`Drop`トレイトを実装する
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("{:?}", x);
    println!("Made a ToDrop!");

    // 整数をヒープ上に確保
    let box2 = Box::new(5i32);
    // `box2`は`box3`にムーブされる
    let box3 = box2;
    println!("{:?}", box3);
    
    // ネストしたスコープ
    {
        // 整数をヒープ上に確保
        let box4 = Box::new(4i32);
        println!("{:?}", box4);

        // `box4`はここでスコープを抜けるため破棄されて、メモリは解放される
    }

    for _ in 0u32..10000 {
        create_box();
    }
    // `box3`はここでスコープを抜けるため破棄されて、メモリは解放される
}
