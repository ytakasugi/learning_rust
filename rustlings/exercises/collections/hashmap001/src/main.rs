use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 1);

    basket
}

fn main() {
    let basket = fruit_basket();
    println!("{:?}", basket.get("banana"));
}
