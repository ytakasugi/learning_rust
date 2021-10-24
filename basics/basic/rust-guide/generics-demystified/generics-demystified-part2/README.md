# [Rust Guide: Generics Demystified Part 2](https://jeffa.io/rust_guide_generics_demystified_part_2)

## INTRODUCTION

パート2へようこそ!

第1部のコードでは、ジェネリックスの力を表面的に表現しただけの、ほぼクールなライブラリができあがりました。このセクションでは、実装とトレイトについて説明します。

パート1の最後の方で、`Fn`トレイトを使いましたが、Boxの中で`dyn`と一緒に使いました。`dyn`キーワードは、アンダーザフッドのランタイム抽象化を使用していることを意味し、`Box`はヒープの割り当てを示しています。Rustはコンパイル時の利便性を重視しているので、型システムの最も強力な部分をより深く掘り下げていく中で、`Box<dyn Fn<T>>`を廃止していきます。

その前に、第1部の簡単なテクニックが終わったところで、何を勉強しているのか、なぜそれが重要なのかについて、同じページにいることを確認しましょう。

## WHY DO WE NEED MORE COMPLEXITY?

第1部のプログラムを見ると、`Point`にはどのような`Id`でもよく、`Line`は正確に2つの`Point`を持たなければならず、`Connector`は2つの`Point`で実行して1つの`Line`を生成する限り、ライブラリユーザが実行時に望むことは何でもできることがわかりました。メイン関数では、デバッグフォーマッタを使用して、`Line`インスタンスをコンソールに出力しました。しかし、すべての`Connector`が`Debug`フォーマッタで表示可能なラ`Line`（例：`{:?}`）を生成することを保証したい場合はどうすればよいでしょうか？これが常に機能するように選択するにはどうすればよいでしょうか。また、`Debug`タイプの使用を強制せずに動作させたい場合はどうすればよいでしょうか？これが選択的柔軟性ということです。

私たちは、`struct Line<Id: Debug＞`と書くことができます。このトレイト境界により、`Id`型は`Debug`トレイトを実装したものに制限されます。しかし、型定義を可能な限りシンプルにすることには、やむを得ない理由があります。最も重要なことは、すべてのフィールドがpublicでないと、そのモジュールの外から型を構築することができないということです。そのため、少なくとも1つの関連する関数を使用しなければ、その型に対して何もすることができず、新しいメソッドが必要になります。これがトレイト境界がほとんどの場合、型定義ではなく`impl`ブロックに書かれる理由です。

`std::iter`モジュールには、クロージャを保持するように設計された多くの型がありますが、それがどのようなクロージャであるかを知るためには、関連する`impl`（例えば、`Map`のための`impl`）を見なければなりません。何かをするために標準ライブラリのコードを読んでいるときは、自分が正しいことをしていると思います。しかし、段階的に作業を進め、自分が書いているものを常に理解するようにしてください。これからそのプロセスを見ていきましょう。

## IMPLEMENTATIONS

型の定義（`struct`または`enum`）は、それが何であるかを説明します。型の実装（`impl`）は、その型が何をできるかを記述します。型をパラメタ化して`impl`で制限すると、その`impl`の条件を満たしたときに型ができることが記述されます。

まずは上で説明した、`Id`が`Debug`可能な場合に`Line`が`Debug`可能になるようにすることから始めましょう。

```rust
use std::fmt::{self, Debug, Formatter};

// Remember to remove `#[derive(Debug)]` from `Line`.
impl<Id: Debug> Debug for Line<Id> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Line").field("points", &self.points).finish()
    }
}
```

ここで重要なのは、`Debug`ではない型を保持できる柔軟性が残っているということです。この実装では単に、`Line`は`Id`が`Debug`のメソッドを使用できる場合に限り、`Line`も`Debug`のメソッドを使用できる "というだけです。一方、`struct Line<Id: Debug>`であれば、`Id`が`Debug`を実装していないと`Line`は存在できないということになります。私たちは、`#[derive(Debug)]`が裏でやっていることをやっただけです。これは魔法ではなく、クリエイティブなコーディングなのです。

今後は、自分たちの目標を明確にする必要があります。Rustの選択的な柔軟性と明示性の組み合わせの素晴らしい点は、ツールが理解され、目標が明確であれば、コードは実質的に自力で書けるということです。

`Connector`をイテレータにして、`Point`インスタンスのコレクションから`Line`インスタンスを生成できるようにします。その後、2つだけでなく、任意の数のポイントを使用して、`Line`だけでなく、あらゆる種類のトレイトを作成できるようにしていきます。しかし、コードを完全に理解できるように、段階的に進めていきます。

第2部の終わりには、このライブラリのユーザーは、いくつかのポイントを任意のコレクション（\Vec\だけではない）に入れて、メソッドを呼び出します。そして、ポイントをクロージャーで結合する方法をメソッドに伝えます。これは、標準ライブラリが`collect`メソッドを使ってイテレータをコレクションにしたり、`map`メソッドを使ってイテレータの型や値を変更したりするのと似ています。

まず、イテレータを標準ライブラリのイテレータに近づけることから始めましょう。つまり、`Box<dyn Fn>`を無制限の汎用型に置き換え、`impl`ブロックで`Fn`トレイトを実装した型を指定します。いつものように、標準ライブラリは物事を正しく行う方法の最良の例を提供してくれます。

```rust
struct Connector<Id, F> {
    points: Vec<Point<Id>>,
    op: F,
}

impl<Id, F> Connector<Id, F>
where
    F: Fn(Point<Id>, Point<Id>) -> Line<Id>,
{
    fn new(points: Vec<Point<Id>>, op: F) -> Self {
        Connector { points, op }
    }
}
```

コードが少しクリーンになり、パフォーマンスが向上したこと以外、特に変わったことはありません。ランタイムのオーバーヘッドが減ったのは良いことです。しかし、ユーザは間違ったタイプの`Connector`を作成することはできません。なぜなら、トレイト境界が指定された`impl`ブロックの中にある`new`を使用しなければならないからです。後に、同様の `impl`ブロックでしか`Connector::new`を呼び出さないことが判明した場合、この特定の trait バインドを削除することができます。なお、標準ライブラリでは、`Map::new`が他の場所で誤用されないように、`pub(in crate::iter)`を使用しています。さて、`points`フィールドに使われている`Vec`について説明します。

これは標準ライブラリでは行われていない方法です。このイテレータは実際に`Vec`を所有しています。これは、他のコレクションではなく`Vec`型を使用するという不必要な制限を課しているだけでなく、コレクションの内容だけでなく、コレクション自体の所有権を取得しています。これは、私たちが何をしているのかを理解するための予備的なステップです。標準ライブラリの`iter`モジュールでのジェネリックなトレイトの使い方は見事ですが、簡単に真似することができます。このステップでは、必要なレベルの柔軟性を生み出す作業に移る前に、正しい動作をしていることを確認します。

```rust
impl<Id, F> Iterator for Connector<Id, F>
where
    F: Fn(Point<Id>, Point<Id>) -> Line<Id>,
{
    type Item = Line<Id>;

    fn next(&mut self) -> Option<Self::Item> {
        let point_a = self.points.pop()?;
        let point_b = self.points.pop()?;
        let line = (self.op)(point_a, point_b);

        Some(line)
    }
}
```

## TRAITS

トレイトはそれ自体が型システムの抽象化である。自作のトレイトとトレイトに束縛されたジェネリックを組み合わせることは、マスターするための不可欠なステップです。

まずはトレイトを使って`Vec`型のメソッドを実装してみましょう。これは、イテレータに対して`map`や`filter`を呼び出すのに似ています。一つのメソッドを持つ trait の名前は、そのメソッドの後に付けるのが「ベストプラクティス」とされています。そこで、1メソッドのトレイトを単に`Connect`と呼ぶことにします。このトレイトは`connect`関数を置き換えるものです。

`impl<T, U>`と書くと、その`impl`ブロックのスコープ内にあるジェネリック型を宣言していることになります。これらの汎用型を型パラメータとしてトレイトに渡さなければなりません。トレイトを見ると、その理由がわかります。これらの型パラメタは、コンパイラに対して、その トレイトのメソッドでどのような型を期待するかを教えてくれます。これでコンパイラは、私たちの実装が`connect`メソッドを正しく使っていることを知ることができます。

```rust
trait Connect<Id, F>
where
    F: Fn(Point<Id>, Point<Id>) -> Line<Id>,
{
    fn connect(self, op: F) -> Connector<Id, F>;
}

impl<Id, F> Connect<Id, F> for Vec<Point<Id>>
where
    F: Fn(Point<Id>, Point<Id>) -> Line<Id>,
{
    fn connect(self, op: F) -> Connector<Id, F> {
        Connector::new(self, op)
    }
}

fn main() {
    let points = vec![Point { id: 1001 }, Point { id: 1002 }];
    let mut connector = points.connect(|left, right| Line {
        points: (left, right),
    });

    print!(
        "This should be a line: {:?}.\nThis should be nothing: {:?}.",
        connector.next(),
        connector.next()
    )
}
```

ここで、`Connector`イテレータの作成を終了します。このイテレータは、別のイテレータ（`Vec`だけではない）で `connect`を呼び出して作成します。標準ライブラリの類似した型についていくつかの見解を述べ、その見解を私たちのコードに適用してみましょう。

1. `Map`型は非常に柔軟な型定義を持っており、境界のないジェネリックを持っています。新しいメソッドも境界がありませんが、これは`std::iter`の中でしか使えません。

   これを`Connector`に当てはめてみましょう。

   ```rust
    struct Connector<I, F> {
        points_iter: I,
        op: F,
    }
   
    impl<I, F> Connector<I, F> {
        fn new(points_iter: I, op: F) -> Self {
            Connector { points_iter, op }
        }
    }
   ```

2. 一般的に、`std::iter`型は、内部のイテレータに対して`next`を呼び出し、その動作を単純に変更します。Take::nextのような多くの型は、非常にシンプルなコードでこれを行います。

   また、標準ライブラリは、`このimpl`ブロックが厳密に必要な場所であるため、traitの境界を課すべきだと教えてくれています。`Iterator`型には、`Item`という関連する型があることに注意してください。`<Item = Something>`という構文により、汎用的な型と区別されます。

   これは、単に`Vec`からアイテムを取得していた古いコードを置き換えるものです。これは、`Item`が`Point<Id>`であるイテレータを作成できるすべての型で動作します。

   ```rust
    impl<Id, I, F> Iterator for Connector<I, F>
    where
        I: Iterator<Item = Point<Id>>,
        F: Fn(Point<Id>, Point<Id>) -> Line<Id>,
    {
        type Item = Line<Id>;
   
        fn next(&mut self) -> Option<Self::Item> {
            let point_a = self.points_iter.next()?;
            let point_b = self.points_iter.next()?;
            let line = (self.op)(point_a, point_b);
   
            Some(line)
        }
    }
   ```

3. イテレータは、他のイテレータを生成するメソッドを持っています。一つの例だけでは不十分で、他にも何十もの例が `Iterator`トレイトにはあります。

   まず、`Connect`トレイトを変更する必要があります。なぜなら、そのメソッドは`Connector`を返すので、すでに変更されているからです。`Connector`をより柔軟に書き換えたので、`Connect`にも同じことをしましょう。今後も、トレイトの境界を定義から実装に移すというパターンを続けていきます。

   ```rust
    trait Connect<I, F> {
        fn connect(self, op: F) -> Connector<I, F>;
    }
   ```

   イテレータにメソッドを追加することはできません、それはコードの一部ではないからです。しかし、非常に便利なトリックを使うことができます。`Iterator`を実装するすべての型に対して、私たちの`Connect`メソッドを実装するのです。

   ```rust
   impl<I, Id, F> Connect<Id, F> for I
   where
       I: Iterator,
   {
       fn connect(self, op: F) -> Connector<I, F> {
           Connector::new(self, op)
       }
   }
   ```

   これを「自動実装」といいます。標準ライブラリでは、この自動実装が多用されています。よく知られている例では、ある型に対して`Display`を実装すると、その型に`ToString`の自動実装が施されます。

   `Fn`トレイト境界も、`Connect`と`Connector`にはもう必要ないので削除されました。選択的な柔軟性のおかげで、私たちのコードはエレガントに、よりシンプルに、よりパワフルになりました。それでは、新しいコードを実際に見てみましょう。

   ```rust
   fn main() {
       let points = {
           let mut map = std::collections::HashMap::new();
   
           map.insert("A", Point { id: 1001 });
           map.insert("B", Point { id: 1002 });
           map.insert("C", Point { id: 1003 });
           map.insert("D", Point { id: 1004 });
           map.insert("E", Point { id: 1005 });
   
           map
       };
   
       let mut connector = points.into_values().connect(|left, right| Line {
           points: (left, right),
       });
   
       println!("This should be a line: {:?}.", connector.next());
       println!("This should be a line: {:?}.", connector.next());
       println!("This should be nothing: {:?}.", connector.next());
   }
   ```

   [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=88ebb7e463b50353ff046e41e9b64ebf)

私たちのコードは、`Vec`の`Connect`を手動で実装したときよりもはるかに多くのことを行っていますが、使用した行数はまったく同じです。`std::collections`のすべての型に対して`Connect`を手動で実装するのに必要なコードの量を想像してみてください。ジェネリックスの洗練された技術のおかげで、たった1つのコードブロックで同じことができ、コンパイラが空白を埋めてくれます。

## CONCLUSION

パート2では、少し時間をかけて実験しないと定着しないかもしれない重要な観察事項が満載でした。ありがたいことに、この部分は難しい部分でした。これで、自分のコードで型パラメタを使えるようになったというコンディフィケーションを感じてもらえたと思います。しかし、型パラメータはジェネリックの一部に過ぎません。型を理解した後は、このトピックの最後の部分に進むのは難しくありません。

第3部では、`const`ジェネリクスとパラメタ化されたライフタイムを扱います。

