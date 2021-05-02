### [Returning Traits with `dyn`](https://doc.rust-jp.rs/rust-by-example-ja/trait/dyn.html#returning-traits-with-dyn)

`Rust`コンパイラは、各関数の戻り値の型がどれだけのスペースを必要とするかを知る必要があります。つまり、すべての関数は具体的な型を返さなければなりません。他の言語とは異なり、`Animal`のようなトレイトを持っている場合、`Animal`を返す関数を書くことはできません。

しかし、簡単な回避策があります。トレイトオブジェクトを直接返すのではなく、`Animal`を含む`Box`を返すようにします。`Box`はヒープ内のメモリへの参照にすぎません。参照は静的に既知のサイズを持ち、コンパイラはそれがヒープに割り当てられた`Animal`を指していることを保証できるので、関数からトレイトを返すことができます。

`Rust`は、ヒープ上のメモリを確保する際には、可能な限り明示的にしようとします。そのため、もしあなたの関数がこの方法でヒープ上のトレイトへのポインタを返すのであれば、戻り値の型を`dyn`キーワードで書く必要があります。

~~~rust
struct Sheep {}
struct Cow {}

trait Animal {
    // インスタンスメソッドシグネチャ
    fn noise(&self) -> &'static str;
}

// `Sheep`構造体に`Animal`トレイトを実装する
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// `Cow`構造体に`Animal`トレイトを実装する
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// `Animal`を実装した構造体を返しますが、コンパイル時にどの構造体かはわかりません。
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
~~~