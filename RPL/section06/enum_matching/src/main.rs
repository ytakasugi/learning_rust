#![allow(unused_variables)]
fn main() {
    //enum IpAddrKind {
    //    V4,
    //    V6,
    //}

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    //struct IpAddr {
    //    kind: IpAddrKind,
    //    address: String,
    //}

    //fn route(ip_type: IpAddrKind) {}

    //route(IpAddrKind::V4);
    //route(IpAddrKind::V6);

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    //let home = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from("127.0.0.1")
    //};

    let home = IpAddr::V4(127, 0, 0, 1);

    //let lookback = IpAddr {
    //    kind: IpAddrKind::V6,
    //    address: String::from("::1"),
    //};

    let lookback = IpAddr::V6(String::from("::1"));

    enum Message {
        //Quit,
        //Move {x: i32, y: i32},
        Write(String),
        //ChangeColor(i32, i32, i32)
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    //enum Option<T> {
    //    Some(T),
    //    None,
    //}

    //let sum_number = Some(5);
    //let some_string = Some("a string");
    //let absent_number: Option<i32> = None;

//    #[derive(Debug)]
//    enum UsState {
//        Alabama,
//        Alaska,
//    }


//    enum Coin {
//        Penny,
//        Nickel,
//        Dime,
//        Quarter(UsState),
//    }

//    fn value_in_cests(coin:Coin) -> u32 {
//        match coin {
//            Coin::Penny => {
//                println!("Lucky Penny!");
//                1
//            },
//            //Coin::Penny => 1,
//            Coin::Nickel => 5,
//            Coin::Dime => 10,
//            Coin::Quarter(state) => {
//                    println!("State quarter from {:?}, state");
//                    25
//            },
//        }
//    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("number is {:?}", five);
    println!("number is {:?}", six);
    println!("number is {:?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
    //match some_u8_value {
    //    Some(3) => println!("three"),
    //    _ => (),
    //}
}