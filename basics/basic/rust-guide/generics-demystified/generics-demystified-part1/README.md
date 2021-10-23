# [Rust Guide: Generics Demystified Part 1](https://jeffa.io/rust_guide_generics_demystified_part_1)

## Introduction

Rustaceansがジェネリックを評価する理由は3つあります。

- ジェネリクスはコンパイル時の抽象化です。ジェネリックはコンパイル時の抽象化であり、ジェネリックがしばしば置き換えることのできる`dyn`キーワードは、実行時のオーバーヘッドがあります。

- ジェネリクスはコードをよりクリーンにし、再利用性を高めます。トレイト境界のおかげで、コードはその境界が満たされたときに何ができるかを記述することができます。コードは、あなたが望むのと同じくらい柔軟になります。

- ジェネリックスは、ライフタイムを理解するのに役立ちます。本書では、ライフタイムを命令型のコード（つまり、関数本体のコード）で説明しています。しかし、ジェネリック・ライフタイムは宣言的な文脈を持っています。これにより、型やトレイト、実装の中で使われたときに明確な意味を持ちます。

それでは、ジェネリックについて知っておくべきことをすべて学んでいきましょう。徐々に学んでいくことで、レッスンの各部分を消化してから次のレッスンに進むことができます。

## THE GOAL

いくつかの簡単な例を見ていくことで、『The Book』を読んだ人が自信を持ってジェネリックを使えるようになります。ジェネリクスを使うと、選択的な柔軟性を持ったコーディングが可能になります。このツールを理解すれば、必要に応じて型を追加するという概念が自然に身につきます。

非常にシンプルな例から始めて、型の柔軟性を導入することで、ジェネリック（またはパラメタ化された）型が行うべきことを正確に行います。その後、徐々に複雑さを増していき、ジェネリックが型システムのあらゆる側面に浸透している理由、任意の制限が必要な理由、コードを不可解なグープにすることなくこの強力なツールを使用する方法を学びます。最終的には、Rustの型システムを完全にコントロールできるようになります。

このガイドでは、小さなライブラリを書くプロセスを、豊富なコード例とともに説明します。各パートは30分以内に完成させることができます。フォローしていただければ、各パートの後にコードがコンパイルされます。パート1では、型、関数、クロージャの基本を説明します。第2部では、実装とtraitを掘り下げます。最後に、パート3では`const`ジェネリクスとパラメター化されたライフタイムを追加します。このようにして完成したライブラリは、シンプルでありながら楽しく、理解しやすいものになります。

パート1では、主に「The Book」で取り上げられた情報を復習しますが、いくつかの素晴らしい意見もあります。ご質問、ご意見、ご感想などは、ホームページにある連絡先にお願いします。



## THE BASICS

### TYPE DEFINITIONS

複数のジェネリックをあらゆるレベルで使用するような手の込んだ実装をする前に、本題に入りましょう。

```rust
struct Point {
    id: u64
}
```

これはおなじみの領域で、u64はハードコードされた（ジェネリックではない）型です。柔軟性はありませんが、シンプルで選択性に優れています。

```rust
struct Point<Id> {
    id: Id
}
```

この`Point`は、他の型を`Id`として使用することができます。`Id`は`Point`や`Point`の`Vec`であっても構いませんが、これはかなり無茶な話です。`Point<Vec<Point<()>>`



この柔軟性は過剰になる可能性があるため、後に実装を使用する際に`where`節が役立ちます。今のところ、`Point<Id>`型をユーザからは非公開にしておき、意味のある`Id`を持つパブリック型を公開することができます。この型定義に トレイト境界を使用することもできます。しかし、このような方法はほとんど見られません。その理由は、第2部で独自の実装やトレイトを書くときに詳しく説明します。

```rust
pub type Point = inner::Point<u32>;
pub type BigPoint = inner::Point<u128>;

mod inner {
    struct Point<Id> {
        id: Id
    }
}
```

`Id`値をランダムに生成する場合は、u128のような大きな型を使用することで、大量の`BigPoint`インスタンスを生成する際に衝突の可能性を低くすることができます。しかし、`Points`の数が少ない場合は、`u128`ごとに使用する16バイトの代わりに、`u32`型ではわずか4バイトしか使用しません。これは、型を定義する際にジェネリックがどのように役立つかを示す簡単な例です。

ここまでで、3つのレベルの柔軟性を見てきました。

- ハードコードされた（ジェネリックではない）型。
- 柔軟性の高いオープンエンドのジェネリック型。
- 非公開で使用されるジェネリック型で、公開インターフェース用に特定の型がプラグインされているもの。

### FUNCTIONS AND CLOSURES

関数は、型定義（関数シグネチャ）と実行コンテキスト（命令型コードブロック）で構成されています。実装に移る前に、簡単な関数から始めるとよいでしょう。

```rust
struct Line<P> {
    points: (P, P),
}

fn connect<P>(left: P, right: P) -> Line<P> {
    Line {
        points: (left, right)
    }
}
```

ここでは、`connect`が呼び出された場所で`Point`タプルで使用されるタイプが決定されます。なぜなら、そこで`Line`が呼び出されるからです。緊密な結合が許容される場合は、`connect`が常に`Line`と`Point`を扱うように指定し、`Point`のidフィールドに一般的な型を許可することができます。

```rust
fn connect<Id>(left: Point<Id>, right: Point<Id>) -> Line<Point<Id>> {
    Line {
        points: (left, right)
    }
}
```

型のパラメータはまだありますが、この関数では `Point`型と `Line`型が緊密に結合されています。

他の関数と同様に、引数を構造体として記述し、関数本体をメソッドに移すこともできます。ここでは、関数型とオブジェクト指向型のコードの美しさの違いを議論するのではなく、関数のシグネチャが型定義であることを説明します。これは些細な観察ではありません。今のところ、それを指摘するだけで十分です。それでは、クロージャに汎用型を使う方法に移りましょう。

```rust
/// This is the same Point we used to first demonstrate a generic type.
#[derive(Debug)]
struct Point<Id> {
    id: Id
}

/// This Line is different. We've restricted it to use the Point type but left
/// the Id flexible. Before, this tight coupling was done in the `connect`
// method.
#[derive(Debug)]
struct Line<Id> {
    points: (Point<Id>, Point<Id>),
}

/// This new type contains a closure. While Box is a type, this involves Fn, a
/// trait that uses generic types. We will write our own such trait later.
struct Connector<Id> {
    pub op: Box<dyn Fn(Point<Id>, Point<Id>) -> Line<Id>>,
}

fn main() {
    let connector: Connector<String> = Connector {
        op: Box::new(|left, right| Line {
            points: (left, right),
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
```

## CONCLUSION

ここまで紹介してきたツールで、必要なものはすべて揃っているように見えるかもしれません。確かに、このいくつかのシンプルなツールで多くのことができるのは事実です。しかし、Rustにはジェネリックの力を存分に発揮するために、もっと多くの構文が必要です。パート2は今回のレッスンよりも複雑になりますが、言語全体としては理解しやすくなるでしょう。

このガイドについてご質問やご意見がありましたら、連絡先はホームページにあります。ご意見、ご感想をお待ちしております。

[パート2へ](https://jeffa.io/rust_guide_generics_demystified_part_2)