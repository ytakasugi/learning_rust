#[allow(dead_code)]
// #[derive(Default)]
struct Polygon {
    vertexes: Vec<(i32, i32)>,
    stroke_width: u8,
    fill: (u8, u8, u8),
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            stroke_width: 1,               // デフォルト値を1にする
            vertexes: Default::default(),  // Vec<(i32, i32)>のDefault実装を使う
            fill: Default::default(),      // (u8, u8, u8)のDefault実装を使う
        }
    }
}

fn main() {
    // すべてのフィールドがデフォルト値を持つPolygonを作成する
    #[allow(unused_variables)]
    let polygon1: Polygon = Default::default();

    println!("polygon1.stroke_width = {:?}", polygon1.stroke_width);
    println!("polygon1.vertexes = {:?}", polygon1.vertexes);
    println!("polygon1.fill = {:?}", polygon1.fill);

    // vertexesフィールドだけ別の値に設定し、他はデフォルト値にする
    #[allow(unused_variables)]
    let polygon2 = Polygon {
        vertexes: vec![(0, 0), (3, 0), (2, 2)],
        .. Default::default()
    };

    println!("polygon2.stroke_width = {:?}", polygon2.stroke_width);

    for i in polygon2.vertexes.iter() {
        println!("polygon2.vertexes = {:?}", i);
    }

    println!("polygon2.fill = {:?}", polygon2.fill);
}