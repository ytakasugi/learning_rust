struct Bicycle {
    cadence: i32,
    speed: i32,
    gear: i32,
}

// 各メソッドが状態を変更できるようにするには、シグネチャに`&mut self`を含める必要がある
impl Bicycle {
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

fn main() {
    // メソッドを呼び出して状態を変更するので、可変変数として宣言
    let mut bike1 = Bicycle { cadence: 0, speed: 0, gear: 0};
    bike1.change_cadence(50);
    bike1.speed_up(10);
    bike1.change_gear(2);
    bike1.print_states();

    let mut bike2 = Bicycle { cadence: 0, speed: 0, gear: 0 };
    bike2.change_cadence(50);
    bike2.speed_up(10);
    bike2.change_gear(2);
    bike2.print_states();
    // 新しい値へ上書きする
    bike2.change_cadence(40);
    bike2.speed_up(10);
    bike2.change_gear(3);
    bike2.print_states();
}
