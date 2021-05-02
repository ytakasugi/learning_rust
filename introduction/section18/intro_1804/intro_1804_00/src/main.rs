fn double_first(vec: Vec<&str>) -> i32 {
    // Generate error 1
    // first():スライスの最初の要素、または空の場合は`None`を返します。
    let first = vec.first().unwrap();
    // Generate error 2
    2 * first.parse::<i32>().unwrap()
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));
    // Error 1: the input vector is empty
    println!("The first doubled is {}", double_first(empty));

    // Error 2: the element doesn't parse to a number
    println!("The first doubled is {}", double_first(strings));
}

