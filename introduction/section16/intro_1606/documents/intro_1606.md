### [`impl Trait`](https://doc.rust-jp.rs/rust-by-example-ja/trait/impl_trait.html#impl-trait)

関数が`MyTrait`を実装した型を返す場合、その戻り値の型を`-> impl MyTrait`と書くことができます。これは、型のシグネチャを非常に簡単にするのに役立ちます。

~~~rust
use std::iter;
use std::vec::IntoIter;

// この関数は2つの `Vec<i32>` を結合し、その上のイテレータを返します。
// その戻り値の型がどれほど複雑か見てください!
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// これは全く同じ関数ですが、戻り値の型は `impl Trait` を用います。
// 見てください、こんなにシンプルになりました
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
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
}
~~~

さらに重要なことに、`Rust`の型の中には書き出すことができないものがあります。例えば、すべてのクロージャは、それ自身の名前のない具象型を持っています。`impl Trait`構文の前は、クロージャを返すためにヒープ上で割り当てなければなりませんでした。しかし、現在では、以下のように静的にすべてを行うことができます。

~~~rust
// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn main() {
    let plus_one = make_adder_function(1);
    assert_eq!(plus_one(2), 3);
}
~~~

また、マップやフィルタのクロージャを使用するイテレータを返すために`impl Trait`を使用することもできます！これにより、マップやフィルタの使用がより簡単になります。クロージャの型には名前がないので、関数がクロージャを使用したイテレータを返す場合、明示的な戻り値の型を書き出すことはできません。しかし`impl Trait`を使えば、これを簡単に行うことができます。

~~~rust
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}
~~~
