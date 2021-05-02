#[derive(Debug)]

struct Person<'a> {
    //The 'a is define a lifetime
    name: &'a str,
    age: u8,
}

// a unit struct
struct Nil;

// a tuple struct
struct Pair (i32, f32);

// a struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Struct can be reuse as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // a rectangle can be specified by where the top left and bottom right
    // corner are in space
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = "Perter";
    let age = 27;
    let peter = Person{name, age};

    // Print debug struct
    println!("{:?}", peter);

    // Instance a 'Pont'
    let point: Point = Point {x:10.3, y:0.4};

    // Access the fileds of the Point
    println!("point cordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point{x:5.2, ..point};

    // 'bottom_right' will be the same as 'point.y' because we used that field from 'pont'
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a 'let' binding
    let Point {x:top_edge, y: left_edge} = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left:Point {x:left_edge, y:top_edge},
        bottom_right:bottom_right,
    };

    // Instance a unit struct
    let _nil = Nil;

    // Instance a tuple struct
    let pair = Pair(1, 0.1);

    // Access the the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);


}
