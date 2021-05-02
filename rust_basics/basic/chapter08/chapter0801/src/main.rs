// デカルト座標
struct CartesianCoord {
    x: f64,
    y: f64
}

// 極座標
//         y
//         ^
//         |
// r sin θ +   +
//         |  / r
//         | /
//         |/)θ
//         +---+--------> x
//        O    r cos θ

struct PolarCoord {
    r: f64,
    theta: f64,
}

// トレイトを定義
trait Coordinates {
    // ここに関数本体は書かない
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

// デカルト座標系はそのまま
impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

// 極座標系は変換が必要
impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x * cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn main() {
    // 値を用意する
    let point = (1.0, 1.0);

    // トレイトのメソッドを呼ぶ
    let c = point.to_cartesian();
    println!("x = {}, y = {}", c.x, c.y);

    // 同じくトレイトの関連関数を呼ぶ(後述
    let p: PolarCoord = Coordinates::from_cartesian(c);
    println!("r = {}, θ = {}", p.r, p.theta);
}