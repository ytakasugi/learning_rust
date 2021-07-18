#[allow(unused)]
enum Bicycle {
    Basic {
        cadence : i32, 
        speed: i32, 
        gear: i32
    },
    Mountain {
        cadence : i32, 
        speed: i32, 
        gear: i32, 
        tires: String
    }
}

#[allow(unused)]
impl Bicycle {
    fn print_states(&self) {
        match self {
            // There's even a shorthand for matching multiple variants with the same implementation 
            Bicycle::Basic { cadence, speed, gear } | Bicycle::Mountain { cadence, speed, gear, .. } => {
                println!("cadence: {} speed: {} gear: {}", cadence, speed, gear)
            }
        }
    }

    fn speed_up(&mut self, increment : i32) {
        match self {
            Bicycle::Basic { ref mut speed, .. } | Bicycle::Mountain { ref mut speed, ..} => {
                *speed += increment
            }
        }
    }
}

#[test]
fn random_bike() {
    let coin : bool = rand::random();
    let mut bike;
    if coin {
        bike = Bicycle::Basic { cadence: 0, speed: 0, gear: 0};
    } else {
        bike = Bicycle::Mountain { cadence: 0, speed: 0, gear: 0, tires: "mountain tires".to_string()};
    }
    bike.speed_up(123);
    bike.print_states(); 

}
