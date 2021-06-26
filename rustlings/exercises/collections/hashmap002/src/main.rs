use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lichi,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kind = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lichi,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kind {
        if !basket.contains_key(&fruit) {
            basket.insert(fruit, 1);
        }
    }
}

fn main() {
    let mut basket = HashMap::<Fruit, u32>::new();
    basket.insert(Fruit::Apple, 4);
    basket.insert(Fruit::Mango, 2);
    basket.insert(Fruit::Lichi, 5);

    assert_eq!(basket.len(), 3);

    fruit_basket(&mut basket);
    println!("{}", basket.len());

    let count = basket.values().sum::<u32>();
    println!("count = {}", count);
}
