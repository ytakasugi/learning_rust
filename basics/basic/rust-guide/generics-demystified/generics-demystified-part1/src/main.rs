// これは、ジェネリック・タイプを最初にデモンストレーションしたときに使ったのと同じポイントです。
#[derive(Debug)]
struct Point<Id> {
    id: Id,
}

// このLineは違います。Pointタイプを使用するように制限していますが、
// Idはフレキシブルにしています。
// 以前は、この緊密な結合は `connect` メソッドで行われていました。
#[derive(Debug)]
struct Line<Id> {
    points: (Point<Id>, Point<Id>),
}

// この新しいタイプにはクロージャが含まれています。
// Boxは型ですが、これにはFnという traitが関わっています。
// このような traitは後で独自に作成する予定です。
struct Connector<Id> {
    pub op: Box<dyn Fn(Point<Id>, Point<Id>) -> Line<Id>>,
}


fn main() {
    let connector: Connector<String> = Connector {
        op: Box::new(|left, right| Line { 
            points: (left, right) 
        }),
    };

    let point_a = Point {
        id: "A".to_string(),
    };

    let point_b = Point {
        id: "B".to_string(),
    };

    let line = (connector.op)(point_a, point_b);

    println!("{:?}", line);
}
