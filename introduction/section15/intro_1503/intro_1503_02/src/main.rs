struct Point {
    x: i32,
    y: i32,
    z: i32
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // データは元々の持ち主と参照の両方からアクセスすることができます。
    println!("Point has coordinates: ({}, {}, {})",
            borrowed_point.x
            , another_borrow.y
            , point.z
    );

    // pointはすでにイミュータブルに借用されているため、ミュータブルに借用することができない。
    // let mutable_borrow = &mut point;

    // ここでも借りた値を使用しています。
    println!("Point has coordinates: ({}, {}, {})",
            borrowed_point.x
            , another_borrow.y
            , point.z
    );

    // 不変参照は残りのコードでは使われなくなっているので、可変参照を使って再利用することができます。
    let mut mutable_borrow = &mut point;

    // ミュータブルなリファレンスを介してデータを変更する
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // `point`はすでにミュータブルに借用されているため、 イミュータブルに借用することはできない。
    // let y = &point.y;

    // `println!`は不変参照を取るため、printできません。
    // println!("Point Z coordinate is {}", point.z);

    // 可変参照は、不変参照として`println!`できる
    println!("Point has coordinates: ({}, {}, {})",
            mutable_borrow.x, 
            mutable_borrow.y, 
            mutable_borrow.z
    );

    // 可変参照は、使われなくなったので再借用することができます。
    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
            new_borrowed_point.x, 
            new_borrowed_point.y, 
            new_borrowed_point.z
    );
}
