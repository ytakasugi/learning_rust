use std::string::String;

fn main() {
    let dog = Dog{};
    let cat = Cat{};
    show_animal_data(dog);
    show_animal_data(cat);
}

trait Animal {
    // Return the life of the animal.
    fn lifespan(&self) -> u32;
    // Returns the academic name of the animal.
    fn scientific_name(&self) -> String;
}

// Define the dog structure.
struct Dog;

// Define `lifespan ()` and`scientific_name()` functions for dog structs.
impl Animal for Dog {
    fn lifespan(&self) -> u32 {
        13
    }

    fn scientific_name(&self) -> String {
        "Canis lupus familiaris".to_string()
    }
}

// Define the cat structure.
struct Cat;

// Define `lifespan ()` and`scientific_name()` functions for cat structs.
impl Animal for Cat {
    fn lifespan(&self) -> u32 {
        16
    }

    fn scientific_name(&self) -> String {
        "Felis catus".to_string()
    }
}

// A function that outputs the animal lifespan and academic name to standard output.
// Call the `lifispan ()` function and `scientific_name ()` defined in the `Animal` trait internally.
// `T: Animal` means to have a` Animal` trait as a boundary.
// This allows only types that implement the `Animal` trait to be included in this` T` type argument.

fn show_animal_data<T: Animal>(animal: T) {
    println!("Lifespan: {} years", animal.lifespan());
    println!("Scientific name: {}", animal.scientific_name());
}




