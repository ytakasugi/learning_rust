/*
「ベクタ」はサイズを変更可能な配列です。
スライスと同様、そのサイズはコンパイル時には不定ですが、いつでも要素を追加したり削除したりすることができます。
ベクタは3つの要素で、その特徴が完全に決まります。
    ・データへのポインタ
    ・長さ
    ・容量(あらかじめメモリ上にベクタのために確保された領域)
ベクタはその容量を超えない限りにおいて長くしていくことができます。超えた場合には、より大きな容量を持つように割り当てなおされます。
*/

fn main() {
    // イテレータは要素を収集してベクタにすることができる
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // ベクタの初期化には`vec!`マクロが使用できる。
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // 新しい要素をベクタの最後に挿入することができる。
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // immutableなベクタには値は追加できない
    // collected_iterator.push(0);

    // `len`メソッドは現在のベクタのサイズを返す。
    println!("Vector length: {}", xs.len());

    // 鍵括弧(`[]`)を用いてインデックスによる要素へのアクセスができる
    // （インデックスは0から開始する）
    println!("Second element: {}", xs[1]);

    // `pop`はベクタの最後の要素を削除すると同時に返す。
    println!("Pop last element: {:?}", xs.pop());

    // 不正なインデックスアクセスはpanicを引き起こす。
    // println!("Fourth element: {}", xs[3]);

    // ベクタは簡単に反復処理ができる
    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    // ベクトル`は、別の変数(`i`)で繰り返し回数を列挙している間にも繰り返し処理を行うことができます。
    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    // `iter_mut` のおかげで、変異可能な`Vector`もまた、各値を変更できるような方法で反復処理することができる。
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);
}

/* 実行結果
Collected (0..10) into: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
Initial vector: [1, 2, 3]
Push 4 into the vector
Vector: [1, 2, 3, 4]
Vector length: 4
Second element: 2
Pop last element: Some(4)
Contents of xs:
> 1
> 2
> 3
In position 0 we have value 1
In position 1 we have value 2
In position 2 we have value 3
Updated vector: [3, 6, 9]
*/