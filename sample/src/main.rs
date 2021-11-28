fn check_size(height: &i32, width: &i32, &depth: &i32) -> bool {
    if 180 >= height + width + depth {
        true
    } else {
        false
    }
}

fn price(height: i32, width: i32, depth: i32, weight: i32) -> i32 {
    let length = height + width + depth;
    let mut  fee = 0;

    if length <= 90 {
        if weight <= 5 {
            fee += 500;
        } else if weight <= 10 {
            fee += 1000;
        } else {
            fee += 1500;
        }
    } else {
        if weight <= 5 {
            fee += 1000;
        } else if weight <= 10 {
            fee += 2000;
        } else {
            fee += 3000;
        }
    }
    fee
}

fn main() {
    let height = 10;
    let width = 10;
    let depth = 10;
    let weight = 10;

    if check_size(&height, &width, &depth) == true {
        println!("{}", price(height, width, depth, weight))
    } else {
        println!("size over");
    }
}