fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I'm not born yet I guess"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }

    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // Match a single value.
        1 => println!("One!"),
        // Match several values.
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases.
        _ => println!("Ain't special"),
    }

    let boolean = true;

    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
