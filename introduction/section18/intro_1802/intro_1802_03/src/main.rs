#![allow(dead_code)]

#[derive(Debug)] 
enum Food { 
    CordonBleu,
    Steak, 
    Sushi 
}

#[derive(Debug)] 
enum Day { 
    Monday, 
    Tuesday, 
    Wednesday 
}

// `Food::Sushi`であれば`None`を、そうでなければ`Some<Food>`を返す関数
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// `Food::CordonBleu`であれば`None`を、そうでなければ`Some<Food>`を返す関数
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBleu => None,
        _ => Some(food),
    }
}

// `have_recipe`が`None`なら`None`を返す。
// `have_recipe`が`Some(food)`かつ`have_ingredients(food)`が`None`なら`None`を、`Some(food)`であれば`Some<Food>`を返す関数
fn cookavle_v1(food: Food) -> Option<Food> {
    match have_recipe(food) {
        None => None,
        Some(food) => match have_ingredients(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// `have_recipe(food)`が`Some(food)`かつ`and_then(have_ingredients)`が`Some(food)`なら`Some<Food>`を返す
fn cookable_v2(food: Food) -> Option<Food> {
    // and_then:オプションが[None]の場合は[None]を、そうでない場合はラップされた値でfを呼び出し、その結果を返します。
    have_recipe(food).and_then(have_ingredients)
}

// `cookable_v2`関数の返り値が`Some(food)`なら一番目のアームを、そうでなければ二番目のアームを返す
fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
        None       => println!("Oh no. We don't get to eat on {:?}?", day),
    }
}

fn main() {
    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    // `have_recipe`関数で`None`となる
    eat(cordon_bleu, Day::Monday);
    // どの関数でも`Some`を返す
    eat(steak, Day::Tuesday);
    // `have_ingredients`で`Noneを返す
    eat(sushi, Day::Wednesday);
}
