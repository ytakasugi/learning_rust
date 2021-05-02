### [New type Idiom](https://doc.rust-jp.rs/rust-by-example-ja/generics/new_types.html)

newtypeイディオムは、正しい型の値がプログラムに与えられることをコンパイル時に保証します。

例えば、年齢を年単位でチェックする年齢確認関数には、Years型の値を与えなければなりません。

~~~rust
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    // 年数の一部を切り捨てます。
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Old enough {}", old_enough(&age));
    println!("Old enough {}", old_enough(&age_days.to_years()));
    // println!("Old enough {}", old_enough(&age_days));
}
~~~

最後のprint文のコメントを外して、指定された型がYearsでなければならないことを確認してください。

newtypeの値を基底型として取得するには、次のようなタプル構文を使用することができます。

~~~rust
struct Years(i64);

fn main() {
    let years = Years(42);
    let years_as_primitive: i64 = years.0;
}
~~~

