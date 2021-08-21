### [std::collections](https://doc.rust-lang.org/stable/std/collections/index.html)

---

コレクション型。

Rustの標準コレクションライブラリは、最も一般的な汎用プログラミングデータ構造の効率的な実装を提供します。標準的な実装を使用することで、2つのライブラリがデータを大幅に変換することなく通信できるようになります。

誤解を恐れずに言えば、`Vec`や`HashMap`を使うべきでしょう。この2つのコレクションは、一般的なデータの保存と処理のほとんどの使用例をカバーしています。これらは非常に優れた機能を持っています。標準ライブラリの他のすべてのコレクションには、それらが最適な選択となる特定の使用例がありますが、それらの使用例は比較してニッチなものです。`Vec`や`HashMap`が技術的に最適ではない場合でも、スタートするには十分な選択肢でしょう。

Rustのコレクションは大きく4つに分類されます。

- シーケンス：[`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html), [`VecDeque`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html), [`LinkedList`](https://doc.rust-lang.org/stable/std/collections/struct.LinkedList.html)
- マップ：[`HashMap`](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html), [`BTreeMap`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html)
- セット：[`HashSet`](https://doc.rust-lang.org/stable/std/collections/hash_set/struct.HashSet.html), [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html)
- そのほか：[`BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html)

---

### どのような場合にどのコレクションを使用すべきか？

これらは、各コレクションをどのような場合に検討すべきかについて、かなりハイレベルで簡単に説明したものです。個々のコレクションの長所と短所についての詳細な説明は、それぞれのドキュメントページにあります。

---

### `Vec`

- 後で処理したり、別の場所に送ったりするためにアイテムを集めたいが、保存される実際の値のプロパティは気にしない。
- 特定の順序で要素のシーケンスが必要であり、最後に（またはその近くに）追加するだけである。
- スタックが必要
- サイズ変更可能な配列が必要
- ヒープアロケートされた配列が欲しい

---

### `VecDeque`

- シーケンスの両端に効率的な挿入をサポートする`Vec`が欲しい。
- キューが欲しい。
- 両端のあるキュー（`deque`）が欲しい。

---

### `LinkedList`

- サイズがわからない`Vec`や`VecDeque`が欲しいが、償却ができない。
- リストの分割や追加を効率的に行いたい。
- 二重連結リストが欲しいと思っている。

---

### `HashMap`

- 任意のキーと任意の値を関連付けたい。
- キャッシュが欲しい。
- 余分な機能を省いたマップが欲しい。

---

### `BtreeMap`

- マップをキーでソートしたい。
- 必要に応じて、エントリの範囲を取得したい。
- 最小または最大のキーと値のペアが何かに興味がある。
- 何かよりも小さいまたは大きい最大または最小のキーを見つけたい。

---

### 次のような場合には、これらの`Map`のうち`Set`のバリエーションを使用します。

- どのキーを見たかを覚えておきたいだけ。
- キーと関連付ける意味のある価値はありません。
- ただセットで欲しいだけです。

---

### `BinaryHeap`

- たくさんの要素を保存したいが、いつでも「最大」または「最も重要」なものだけを処理したい場合。
- 優先度の高いキューが必要

---

### Performance

実施したいことに適したコレクションを選ぶには、それぞれのコレクションが何を得意としているかを理解する必要があります。ここでは、特定の重要な操作に対する各種コレクションの性能を簡単にまとめています。詳細については、各タイプのドキュメントを参照してください。また、コレクションによっては、実際のメソッドの名前が以下の表と異なる場合があります。

このドキュメントでは、いくつかの慣例に従っています。すべての操作において、コレクションのサイズはnで示されます。その操作に別のコレクションが含まれる場合、そのコレクションにはm個の要素が含まれます。償却されるコストを持つ操作には、サフィックスとして`*`が付けられます。予想されるコストを伴う操作には、接尾語として`~`が付く。

すべての償却コストは、容量を使い果たしたときにサイズ変更が必要になる可能性があるためです。サイズ変更が発生した場合は、`O(n)`時間かかります。私たちのコレクションが自動的に縮小することはないので、削除操作は償却されません。十分に大きな一連の操作では、操作ごとの平均コストは、決定論的に与えられたコストと等しくなります。

`HashMap`だけは、ハッシュの確率的な性質のために、期待されるコストを持っています。`HashMap`のパフォーマンスが低下することは、理論的にはあり得ますが、非常に稀なことです。

---

### シーケンス

|                                                              | get(i)           | insert(i)         | remove(i)        | append  | split_off(i)     |
| :----------------------------------------------------------- | :--------------- | :---------------- | :--------------- | :------ | :--------------- |
| [`Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html) | `O(1)`           | `O(n-i)*`         | `O(n-i)`         | `O(m)*` | `O(n-i)`         |
| [`VecDeque`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html) | `O(1)`           | `O(min(i, n-i))*` | `O(min(i, n-i))` | `O(m)*` | `O(min(i, n-i))` |
| [`LinkedList`](https://doc.rust-lang.org/stable/std/collections/struct.LinkedList.html) | `O(min(i, n-i))` | `O(min(i, n-i))`  | `O(min(i, n-i))` | `O(1)`  | `O(min(i, n-i))` |

なお、タイが発生した場合、一般的に`Vec`は`VecDeque`よりも速く、`VecDeque`は`LinkedList`よりも速いと考えられます。

---

## [Maps](https://doc.rust-lang.org/stable/std/collections/index.html#maps)

`Set`では、すべての操作に、同等の「マップ」操作のコストがかかります。

|                                                              | get         | insert      | remove      | range       | append   |
| :----------------------------------------------------------- | :---------- | :---------- | :---------- | :---------- | :------- |
| [`HashMap`](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html) | `O(1)~`     | `O(1)~*`    | `O(1)~`     | `N/A`       | `N/A`    |
| [`BTreeMap`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html) | `O(log(n))` | `O(log(n))` | `O(log(n))` | `O(log(n))` | `O(n+m)` |

---

### コレクションの適正かつ効率的な利用

もちろん、どのコレクションが適しているかを知っていても、すぐに正しい使い方ができるわけではありません。ここでは、一般的な標準コレクションを効率的かつ正しく使用するための簡単なヒントを紹介します。特定のコレクションの使い方に興味がある場合は、そのコレクションのドキュメントで詳細な説明やコード例を参照してください。

---

### 容量管理

多くのコレクションは、「容量」を参照するいくつかのコンストラクタやメソッドを提供しています。これらのコレクションは通常、配列の上に構築されています。理想的には、この配列は、コレクションに格納されている要素だけにぴったり合う大きさですが、コレクションがこれを行うのは非常に非効率的です。もし、バックの配列が常に正確なサイズであれば、要素が挿入されるたびに、コレクションはその要素に合わせて配列を大きくしなければなりません。ほとんどのコンピュータではメモリの割り当てと管理が行われているため、この場合、ほぼ確実に全く新しい配列を割り当て、古い配列から新しい配列にすべての要素をコピーする必要があります。このような作業を毎回行うのは効率的ではないことがお分かりいただけると思います。

そのため、ほとんどのコレクションでは、償却型の割り当て戦略を採用しています。一般的に、コレクションにはかなりの量の未使用のスペースがあり、たまにしか大きくならないようにしています。成長するときには、要素を移動するためにかなり大きな配列を割り当てて、次の成長が必要になるまでしばらく時間がかかるようにします。この戦略は一般的には素晴らしいものですが、コレクションが下位の配列のサイズを変更する必要がなければ、さらに素晴らしいものになるでしょう。残念ながら、コレクション自体にはこれを行うための十分な情報がありません。そのため、ヒントを与えるのは私たちプログラマーの役目です。

`with_capacity`コンストラクタは、指定された数の要素に十分なスペースを確保するようコレクションに指示します。理想的には、指定された数の要素に十分なスペースを確保することですが、実装の詳細によってはこれができない場合もあります。詳細はコレクション固有のドキュメントを参照してください。一般的には、挿入される要素の数が正確に分かっているか、少なくともその数に妥当な上限がある場合に`with_capacity`を使用してください。

大量の要素が入ってくることが予想される場合は、 `reserve`メソッド群を使用して、これから入ってくる要素のためにどれだけのスペースを確保すべきかをコレクションに示唆することができます。`with_capacity`と同様に、これらのメソッドの正確な動作は対象となるコレクションに依存します。

パフォーマンスを最適化するために、コレクションは一般的に自分で縮小することを避けます。コレクションの要素がすぐには増えないと思われる場合や、どうしてもメモリが必要な場合、`shink_to_fit`メソッドは、コレクションの要素を格納できる最小のサイズにバック配列を縮小するように促します。

最後に、コレクションの実際の容量を知りたい場合、ほとんどのコレクションでは、必要に応じてこの情報を照会するためのcapacityメソッドを提供しています。これはデバッグの際や`reserve`メソッドを使用する際に役立ちます。

---

### イテレータ

イテレータは、Rustの標準ライブラリ全体で使われている強力で堅牢なメカニズムです。イテレータは、汎用的で安全かつ効率的で便利な方法で値のシーケンスを提供します。イテレータの内容は通常、遅延的に評価されるので、実際に必要な値だけが実際に生成され、それらを一時的に保存するための割り当ては必要ありません。イテレータは主に`for`ループで使用されますが、多くの関数では、値のコレクションやシーケンスが必要な場合にイテレータを使用します。

標準的なコレクションには、その内容を一括して操作するためのイテレータがいくつか用意されています。ほとんどのコレクションが提供すべき3つの主要なイテレータは、`iter`、`iter_mut`、および`into_iter`です。これらのうちのいくつかは、提供することが不健全または不合理であるコレクションでは提供されません。

`iter`は、最も「自然な」順序でコレクションのすべてのコンテンツへの不変的な参照のイテレータを提供します。`Vec`のようなシーケンスコレクションでは、0から始まるインデックスの昇順でアイテムが出力されることを意味します。`HashMap`のような順序付けされていないコレクションでは、内部表現が最も便利な順序でアイテムが出力されます。これは、コレクションのすべてのコンテンツを読むのに最適です。

```rust
let vec = vec![1, 2, 3, 4];
for x in vec.iter() {
   println!("vec contained {}", x);
}
```

`iter_mut`は、`iter`と同じ順序で`mutable`な参照のイテレータを提供します。これは、コレクションのすべての要素を可変にするのに最適です。

```rust
let mut vec = vec![1, 2, 3, 4];
for x in vec.iter_mut() {
   *x += 1;
}
```

`into_iter`は、実際のコレクションを、その内容を値ごとに表すイテレータに変換します。これは、コレクション自体が不要になり、値が別の場所で必要になった場合に有効です。`extend`と`into_iter`を使うと、あるコレクションのコンテンツを別のコレクションに移すことができます。また、イテレータ自体の`collect`を呼び出すことも、あるコレクションを別のコレクションに変換するための優れた方法です。これらのメソッドは、前のセクションで説明した容量管理ツールを内部的に使用して、可能な限り効率的に実行する必要があります。

```rust
let mut vec1 = vec![1, 2, 3, 4];
let vec2 = vec![10, 20, 30, 40];
vec1.extend(vec2);
```

```rust
use std::collections::VecDeque;

let vec = vec![1, 2, 3, 4];
let buf: VecDeque<_> = vec.into_iter().collect();
```

また、イテレータには、シーケンスに共通のスレッドを実行するための一連のアダプタメソッドが用意されています。アダプタの中には、`map`、`fold`、`skip`、`take`といった関数的なものがあります。コレクションで特に興味深いのは `rev`アダプタで、この操作をサポートするイテレータを逆にします。ほとんどのコレクションでは、逆順に反復する方法としてリバーシブルイテレータが用意されています。

```rust
let vec = vec![1, 2, 3, 4];
for x in vec.iter().rev() {
   println!("vec contained {}", x);
}
```

他のいくつかのコレクションメソッドもイテレータを返して結果のシーケンスを生成しますが、結果を格納するためにコレクション全体を確保する必要はありません。これにより、必要に応じて`collect`や`extend`を呼び出してシーケンスを任意のコレクションに「パイプ」することができるため、柔軟性が最大限に高まります。そうでない場合は、`for`ループでシーケンスをループさせることができます。また、イテレータは部分的に使用した後に破棄することができ、未使用のアイテムの計算を防ぐことができます。

---

### Entries

`entry`APIは、キーがあるかどうかを条件に、マップの内容を操作する効率的なメカニズムを提供することを目的としています。この機能の主な目的は、効率的なアキュムレータ・マップを提供することにあります。例えば、各キーが見られた回数を管理したい場合、そのキーが初めて見られたかどうかを条件としたロジックを実行する必要があります。通常は、検索の後に挿入を行う必要があり、挿入のたびに検索の手間がかかることになります。

ユーザーが`map.entry(&key)`を呼び出すと、マップはキーを検索し、`Entry`enumのバリアントを生成します。

`Vacant(entry)`が出力された場合、キーは見つかりませんでした。この場合、唯一有効な操作は、エントリに値を挿入することです。これが行われると、空いているエントリが消費され、挿入された値への可変型参照に変換されます。これにより、検索自体の有効期間を超えて、値をさらに操作することができます。これは、値が挿入されたばかりかどうかにかかわらず、その値に対して複雑なロジックを実行する必要がある場合に便利です。

`Occupied(entry)`が得られた場合、キーが見つかったことになる。この場合、ユーザーにはいくつかの選択肢があります。占領されたエントリの値を取得、挿入、または削除することができます。さらに、占有されているエントリをその値への可変型参照に変換して、空の挿入の場合と対称性を持たせることもできます。

---

### Example

ここでは、`entry`の主な使用方法を2つご紹介します。まず、値に対して実行されるロジックが些細なものである単純な例。

---

### 文字列中の各文字の出現回数をカウントする

```rust
use std::collections::btree_map::BTreeMap;

let mut count = BTreeMap::new();
let message = "she sells sea shells by the sea shore";

for c in message.chars() {
    *count.entry(c).or_insert(0) += 1;
}

assert_eq!(count.get(&'s'), Some(&8));

println!("Number of occurrences of each character");
for (char, count) in &count {
    println!("{}: {}", char, count);
}
```

値に対して実行するロジックがより複雑な場合は、単純に`entry`APIを使用して値が初期化されることを確認し、その後にロジックを実行することもあります。

---

### Tracking the inebriation of customers at a bar

```rust
use std::collections::btree_map::BTreeMap;

// A client of the bar. They have a blood alcohol level.
struct Person { blood_alcohol: f32 }

// All the orders made to the bar, by client ID.
let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

// Our clients.
let mut blood_alcohol = BTreeMap::new();

for id in orders {
    // If this is the first time we've seen this customer, initialize them
    // with no blood alcohol. Otherwise, just retrieve them.
    let person = blood_alcohol.entry(id).or_insert(Person { blood_alcohol: 0.0 });

    // Reduce their blood alcohol level. It takes time to order and drink a beer!
    person.blood_alcohol *= 0.9;

    // Check if they're sober enough to have another beer.
    if person.blood_alcohol > 0.3 {
        // Too drunk... for now.
        println!("Sorry {}, I have to cut you off", id);
    } else {
        // Have another!
        person.blood_alcohol += 0.1;
    }
}
```

---

### Insert and complex keys

もっと複雑なキーを持っている場合、`insert`を呼び出してもキーの値は更新されません。例えば

```rust
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Foo {
    a: u32,
    b: &'static str,
}

// we will compare `Foo`s by their `a` value only.
impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool { self.a == other.a }
}

impl Eq for Foo {}

// we will hash `Foo`s by their `a` value only.
impl Hash for Foo {
    fn hash<H: Hasher>(&self, h: &mut H) { self.a.hash(h); }
}

impl PartialOrd for Foo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.a.partial_cmp(&other.a) }
}

impl Ord for Foo {
    fn cmp(&self, other: &Self) -> Ordering { self.a.cmp(&other.a) }
}

let mut map = BTreeMap::new();
map.insert(Foo { a: 1, b: "baz" }, 99);

// We already have a Foo with an a of 1, so this will be updating the value.
map.insert(Foo { a: 1, b: "xyz" }, 100);

// The value has been updated...
assert_eq!(map.values().next().unwrap(), &100);

// ...but the key hasn't changed. b is still "baz", not "xyz".
assert_eq!(map.keys().next().unwrap().b, "baz");
```

