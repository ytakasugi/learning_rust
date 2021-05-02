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

            // 2つの`Rc`は、その内側の値が等しい場合、等しい。
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // 値のメソッドを直接使うことができる
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));
        
        println!("--- rc_a is dropped out of scope ---");
    }

    // エラー! `rc_examples` はすでに `rc_a` に移動しています。
    // そして、`rc_a` が削除されると、`rc_examples` も一緒に削除されます。
    // println!("rc_examples: {}", rc_examples);
}
