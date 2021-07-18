trait Bicycle {
    fn get_model() -> String;
    fn get_cadence(&self) -> i32;
    fn get_speed(&self) -> i32;
    fn get_gear(&self) -> i32;
    fn print_states(&self) {
        println!("model: {} cadence: {} speed: {} gear: {}", 
            Self::get_model(), 
            &self.get_cadence(), 
            &self.get_speed(), 
            &self.get_gear()
        );
    }
}

struct BasicBicycle {
    cadence: i32,
    speed: i32,
    gear: i32,
}

#[allow(unused)]
impl BasicBicycle {
    fn change_cadence(&mut self, cadence: i32) {
        self.cadence = cadence;
    }

    fn speed_up(&mut self, increment: i32) {
        self.speed += increment;
    }

    fn change_gear(&mut self, gear: i32) {
        self.gear = gear;
    }

    // 状態を表示するだけなので、各フィールドへの不変の参照で問題なし
    fn print_states(&self) {
        println!("cadence: {} speed: {} gear: {}",
            &self.cadence,
            &self.speed,
            &self.gear
        );
    }
}

struct MountainBicycle {
    cadence: i32,
    speed: i32,
    gear: i32,
}

impl Bicycle for BasicBicycle {
    fn get_cadence(&self) -> i32 {
        self.cadence
    }

    fn get_speed(&self) -> i32 {
        self.speed
    }

    fn get_gear(&self) -> i32 {
        self.gear
    }

    fn get_model() -> String {
        "basic".to_string()
    }
}

impl Bicycle for MountainBicycle {
    fn get_cadence(&self) -> i32 {
        self.cadence
    }

    fn get_speed(&self) -> i32 {
        self.speed
    }

    fn get_gear(&self) -> i32 {
        self.gear
    }

    fn get_model() -> String {
        "mountain".to_string()
    }
}

#[test]
fn random_bike() {
    //let coin : bool = rand::random();
    let mut bike = BasicBicycle { cadence: 0, speed: 0, gear: 0};
    
    /*
    if coin {
        bike = BasicBicycle { cadence: 0, speed: 0, gear: 0};
    } else {
        // the next line won't compile
        // bike = MountainBicycle { cadence: 0, speed: 0, gear: 0, tires: "mountain tires".to_string()};
        // mismatched types
        // expected struct `BasicBicycle`, found struct `MountainBicycle`
    }
    */

    bike.speed_up(123);
    bike.print_states();
}