fn main() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        let array = [10, 20, 30, 40, 50 ,60, 70, 80, 90, 100];

        for index in array.iter() {
            println!("{}", index);
        }

    }
}