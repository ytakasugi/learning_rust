/*
Rustにおいて、すべての値はデフォルトでスタックに割り当てられます。
Box<T>を作成することで、値を ボックス化 、すなわちヒープ上に割り当てることができます。
ボックスとは正確にはヒープ上におかれたTの値へのスマートポインタです。
ボックスがスコープを抜けると、デストラクタが呼ばれて内包するオブジェクトが破棄され、ヒープメモリが解放されます。
ボックス化された値は*オペレータを用いてデリファレンスすることができます。これにより一段と直接的な操作が可能になります。
*/

use std::mem;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64
}

// 矩形は、左上と右下の角がスペース内にある位置で指定できます
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point {
        x: 0.0,
        y: 0.0
    }
}

fn boxed_origin() -> Box<Point> {
    // このPointをヒープ上に割り当て、ポインタを返す
    Box::new(Point { 
        x: 0.0,
        y: 0.0 
    })
}

fn main() {
    // （以下では型を全て明示していますが、必須ではありません。）
    // この変数ははすべてスタック上に割り当てられる。
    let point: Point = origin();

    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point {
            x: 3.0,
            y: -4.0
        },
    };

    // ヒープ上に割り当てられたRectangle
    let boxed_rectangle: Box<Rectangle> = Box::new(
        Rectangle {
            top_left: origin(),
            bottom_right: Point { 
                x: 3.0, 
                y: -4.0 
            },
        }
    );

    // 関数の返り値をボックス化
    let boxed_point: Box<Point> = Box::new(origin());

    // 間にもう一つポインタを挟む
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    // `Point`は、{}バイトのスタック領域を占有する
    println!("Point occupies {} bytes on the stack",
            mem::size_of_val(&point));
     // `Rectangle`は、{}バイトのスタック領域を占有する
    println!("Rectangle occupies {} bytes on the stack",
            mem::size_of_val(&rectangle));

    // box size == pointer size
    // ボックスのサイズはポインタのサイズに等しい
    // Box化された`point`は、{}バイトのスタック領域を占有する
    println!("Boxed point occupies {} bytes on the stack",
            mem::size_of_val(&boxed_point));
    // Box化された`rectangle`は、{}バイトのスタック領域を占有する
            println!("Boxed rectangle occupies {} bytes on the stack",
            mem::size_of_val(&boxed_rectangle));
    // Box化された`Box`は、{}バイトのスタック領域を占有する
            println!("Boxed box occupies {} bytes on the stack",
            mem::size_of_val(&box_in_a_box));

    // `boxed_point`の保持するデータを`unboxed_point`にコピーする
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes on the stack",
            mem::size_of_val(&unboxed_point));
}

/* 実行結果
Point occupies 16 bytes on the stack
Rectangle occupies 32 bytes on the stack
Boxed point occupies 8 bytes on the stack
Boxed rectangle occupies 8 bytes on the stack
Boxed box occupies 8 bytes on the stack
Unboxed point occupies 16 bytes on the stack
*/
