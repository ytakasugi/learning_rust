use std::iter;
use std::vec::IntoIter;


// この関数は2つの `Vec<i32>` を結合し、その上のイテレータを返します。
fn combine_vecs_explicit_return_type(v: Vec<i32>,u: Vec<i32>,) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// これは全く同じ関数ですが、戻り値の型は `impl Trait` を用います。
fn combine_vecs(v: Vec<i32>,u: Vec<i32>,) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 入力に `y` を追加する関数を返します。
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a + std::fmt::Debug {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    let v4 = vec![6, 7, 8];
    let v5 = vec![9, 10];
    let mut v6 = combine_vecs_explicit_return_type(v4, v5);
    assert_eq!(Some(6), v6.next());
    assert_eq!(Some(7), v6.next());
    assert_eq!(Some(8), v6.next());
    assert_eq!(Some(9), v6.next());
    assert_eq!(Some(10), v6.next());
    println!("all done");

    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);

    let v =vec![1, 2, 3];
    println!("{:?}", double_positives(&v))
}
