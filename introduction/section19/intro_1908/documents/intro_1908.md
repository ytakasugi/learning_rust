### [Rc](https://doc.rust-jp.rs/rust-by-example-ja/std/rc.html)

複数の所有権が必要な場合は、`Rc`(Reference Counting)を使うことができます。`Rc`は、`Rc`に包まれた値の所有者の数を意味する参照の数を記録します。

`Rc`の参照数は、`Rc`がクローン化されるたびに1ずつ増え、クローン化された`Rc`がスコープから脱落するたびに1ずつ減ります。`Rc`の参照数が0になると、つまり所有者がいなくなると、`Rc`も値もすべて削除されます。

`Rc`のクローン化は、決してディープコピーではありません。クローニングは、ラップされた値への別のポインタを作成し、カウントをインクリメントします。

```rust
use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();
    {
        println!("--- rc_a is created ---");
        
        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        {
            println!("--- rc_a is cloned to rc_b ---");
            
            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
            
            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
            
            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);
            
            println!("--- rc_b is dropped out of scope ---");
        }
        
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        println!("--- rc_a is dropped out of scope ---");
    }
    
    // Error! `rc_examples` already moved into `rc_a`
    // And when `rc_a` is dropped, `rc_examples` is dropped together
    // println!("rc_examples: {}", rc_examples);
    // TODO ^ Try uncommenting this line
}
```

- 参照

  [`std::rc`](https://doc.rust-lang.org/std/rc/index.html)と[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html)

