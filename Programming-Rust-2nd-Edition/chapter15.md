# [Rust] イテレータ

## はじめに

本稿は、プログラミングRust(第2版)と公式ドキュメントを参照し、イテレータとよく使用するであろうメソッドを個人の勉強のためにアウトプットしたものです。

## イテレータとは

- **イテレータ**とは、`std::iter::Iterator`トレイトを実装する任意の型。
- **イテレート可能**とは、`std::iter::IntoIterator`を実装した任意の型。
- イテレータは**値**を生成する。
- イテレータが生成する値を**アイテム**と呼ぶ。
- あるイテレータが生成するアイテムを受け取るコードは**消費者**と呼ぶ。

## `iter`メソッドと`iter_mut`メソッド

多くのコレクション型が、不変参照を返す`iter`メソッドと可変参照を返す`iter_mut`メソッドを実装している。

- `iter`

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    let mut iter = v.iter();

    assert_eq!(immutable_iter.next(), Some(&1));
    assert_eq!(immutable_iter.next(), Some(&2));
    assert_eq!(immutable_iter.next(), Some(&3));
    assert_eq!(immutable_iter.next(), Some(&4));
    assert_eq!(immutable_iter.next(), None);
}
```

- `iter_mut`

```rust
fn main() {
    let x = &mut [1, 2, 4];
    for elem in x.iter_mut() {
        *elem += 2;
    }
    
    assert_eq!(x, &[3, 4, 6]);
}
```

## `IntoIterator`の実装

ある型が`IntoIterator`を実装していれば、`into_iter`メソッドを呼び出すことができる。forループはこれを用いている。

ほとんどのコレクションは、複数の`IntoIterator`を実装している。これらのメソッドは、それぞれ不変参照(`&T`)、可変参照(`&mut T`)、値そのもの(`T`)をアイテムとして生成するイテレータを返す。

- 不変参照に対する`into_iter`は、アイテムへの不変参照を生成するイテレータを返す(つまり、`Item`型が`&T`のイテレータが返される)。
- 可変参照に対する`into_iter`は、アイテムへの可変参照を生成するイテレータを返す(つまり、`Item`型が`&mut T`のイテレータが返される)
- コレクションの値に対する`into_iter`は、コレクションの所有権を取得し、アイテムを値で返すイテレータを返す(つまり、`Item`型が`T`のイテレータが返される)。アイテムの所有権は消費者に移り、元のコレクションはその過程で消費される。イテレータがドロップされると、元のコレクションに残っていた要素もすべてドロップされ、抜け殻となったコレクションも捨てられる。

## イテレータアダプタ

イテレータは、`IntoIterator`トレイトが提供する多様なアダプタメソッドを利用できる。アダプタは、1つのイテレータを消費し、何らかなの有用な動作を行って、別のイテレータを作る。

## [`std::iter::Iterator::map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

`map`は、個々の要素に対してクロージャを適用するイテレータを生成する。

```rust
fn main() {
    let text = " ponies \n giraffes\niguanas  \nsquid".to_string();
    
    let v: Vec<&str> = text.lines()
                        .map(|s| s.trim())
                        .collect();
    assert_eq!(v, ["ponies", "giraffes", "iguanas", "squid"]);
}
```

## [`std::iter::Iterator::filter`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)

`filter`は、個々の要素のうちの一部を取り除いたイテレータを生成する。取り除く条件は、クロージャで決定する。


```rust
fn main() {
    let text = " ponies \n giraffes\niguanas  \nsquid".to_string();
    
    let v: Vec<&str> = text.lines()
                        .map(|s| s.trim())
                        // `s == "iguanas"`はここで取り除かれる
                        .filter(|s| *s != "iguanas")
                        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);
}
```

## [`std::iter::Iterator::filter_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map)

`filter`と`map`のの両方を行うイテレータを生成する。
返されるイテレータは、与えられたクロージャが`Some(value)`を返す値のみ。

```rust
use std::str::FromStr;

fn main() {
    let text = "1\nfrond .25  289\n3.1415 estuary\n";

    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok()) {
            println!("{:4.2}", number.sqrt());
        }
}
```

## [`std::iter::Iterator::flat_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.flat_map)

`flat_map`は、クロージャが返した任意の個数のアイテム列を結合した列を返す。

```rust
use std::collections::HashMap;

fn main() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["S?o Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }
}
```

## [`std::iter::Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

`fold`は、イテレータをが生成するアイテム列全体に対する累積処理を行う。

```rust
fn total(init: i32, n: i32) -> i32 {
    (0..=n).fold(init, |sum, item| sum + item)
}

fn main() {
    println!("{}", total(0, 3)); // total(0, 3) = 6
}
```

`fold`は、アキュムレータと呼ばれる初期値とふたつの引数をもつクロージャを与える。
上記の例の場合、initを初期値とし、`0..=n`の個々の値を受け取り、クロージャ`|sum, item| sum + item`に合計値と個々の値を与えて呼び出し、クロージャの帰り値が新しい合計値となります。

初期値を0とし、0から3までの合計値を計算する場合は以下の通りとなる。

|要素|sum|item|result|
|:--:|:--:|:--:|:--:|
||0|||
|0|0|0|0|
|1|0|1|1|
|2|1|2|3|
|3|3|3|6|