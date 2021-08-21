### [async_std](https://docs.rs/async-std/1.9.0/async_std/)

`async-std`は、ポータブルな`Rust`ソフトウェアの基盤であり、`Rust`のエコシステムのための、最小限で実戦的な共有抽象化のセットです。`async-std`は、`Future`や`Stream`のような標準型、言語プリミティブに対するライブラリで定義された操作、標準マクロ、I/O、マルチスレッドなどを提供します。

`async-std`は`crates.io`から入手できます。インクルードされた`async-std`は、`use async_std::future`のように、`async_std`というパスで`use`文の中でアクセスできます。

---

### [async_std::stream](https://docs.rs/async-std/1.9.0/async_std/stream/index.html)

- Description

  `Composable`な非同期イテレーション。

  このモジュールは、`std::iter`の非同期版です。

  非同期のコレクションを持っていて、そのコレクションの要素に対して操作を行う必要がある場合、すぐに「ストリーム」に遭遇するでしょう。ストリームはRustのイディオム的な非同期コードで多用されているので、よく理解しておくといいでしょう。

  詳しく説明する前に、このモジュールがどのように構成されているかを説明します。

- Organization

  このモジュールは大きくタイプ別に構成されています。

  - Traitsは核となる部分で、どのようなストリームが存在し、それを使って何ができるのかを定義します。これらの特性のメソッドは、特に時間をかけて勉強する価値があります。
  - 関数は、いくつかの基本的なストリームを作成するのに役立つ方法を提供します。
  - 構造体は、このモジュールの特性のさまざまなメソッドの戻り値の型であることが多いです。通常は、構造体そのものではなく、構造体を生成するメソッドを見ることになるでしょう。その理由については、[ストリームの実装](https://docs.rs/async-std/1.9.0/async_std/stream/index.html#implementing-stream)を参照してください。

  これで終わりです。ストリームについて掘り下げてみましょう。

- Stream

  このモジュールの中心となるのは、[Stream](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html)トレイトです。Streamのコアは次のようなものです。

  ~~~rust
  trait Stream {
      type Item;
      fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
  }
  ~~~

  ストリームには[next](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#tymethod.next)というメソッドがあり、呼ばれると`Poll<[Option]<Item>>`が返されます。` next`は要素がある限り`Ready(Some(Item))`を返し、要素がすべてなくなると`None`を返して反復が終了したことを示します。非同期の解決を待っている場合は、`Pending`が返されます。

  個々のストリームは反復を再開することを選択できるので、`next`を再度呼び出すと、ある時点で`Ready(Some(Item))`を再び返すようになるかもしれませんし、そうならないかもしれません。

  ストリームの完全な定義には、他にも多くのメソッドが含まれていますが、それらは`next`の上に構築されたデフォルトのメソッドなので、無料で手に入れることができます。

  ストリームは合成可能なものでもあり、より複雑な処理を行うために、ストリームを連鎖させるのが一般的です。詳しくは、後述の[アダプター](https://docs.rs/async-std/1.9.0/async_std/stream/index.html#adapters)のセクションをご覧ください。

- The three forms of streaming

  - `stream()`: `&T`を反復して処理します。
  - `stream_mut()`: `&mut T`を反復します。
  - `into_stream()`: `T`を反復処理します。

  `async-std`の様々なものは、必要に応じてこの3つのうちの1つ以上を実装します。

- Implementing Stream

  ストリームの状態を保持する構造体の作成と、その構造体に対する`Stream`の実装です。このモジュールに構造体がたくさんあるのはそのためです。ストリームとイテレータのアダプタごとに構造体があります。

  1から5まで数える`Counter`という名前のストリームを作ってみましょう。

  ~~~rust
  // First, the struct:
  
  /// A stream which counts from one to five
  struct Counter {
      count: usize,
  }
  
  // we want our count to start at one, so let's add a new() method to help.
  // This isn't strictly necessary, but is convenient. Note that we start
  // `count` at zero, we'll see why in `next()`'s implementation below.
  impl Counter {
      fn new() -> Counter {
          Counter { count: 0 }
      }
  }
  
  // Then, we implement `Stream` for our `Counter`:
  
  impl Stream for Counter {
      // we will be counting with usize
      type Item = usize;
  
      // poll_next() is the only required method
      fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
          // Increment our count. This is why we started at zero.
          self.count += 1;
  
          // Check to see if we've finished counting or not.
          if self.count < 6 {
              Poll::Ready(Some(self.count))
          } else {
              Poll::Ready(None)
          }
      }
  }
  
  // And now we can use it!
  let mut counter = Counter::new();
  
  let x = counter.next().await.unwrap();
  println!("{}", x);
  
  let x = counter.next().await.unwrap();
  println!("{}", x);
  
  let x = counter.next().await.unwrap();
  println!("{}", x);
  
  let x = counter.next().await.unwrap();
  println!("{}", x);
  
  let x = counter.next().await.unwrap();
  println!("{}", x);
  ~~~

  これで1から5までがそれぞれの行に表示されます。

  この方法で`next().await`を呼び出すと、繰り返しになってしまいます。`Rust`には、ストリームが`None`になるまで`next()`を呼び出すことができる構造があります。次にそれを見てみましょう。

- While let Loops and IntoStream

  Rustの`while let`ループ構文は、ストリームを反復処理するための慣用的な方法です。以下は`while let`の基本的な例です。

  ~~~rust
  let mut values = stream::repeat(1u8).take(5);
  
  while let Some(x) = values.next().await {
      println!("{}", x);
  }
  ~~~

  この例では、1から5までの数字をそれぞれの行に表示しています。しかし、ここで気づくことがあります。ストリームを生成するためにベクターの何かを呼び出していません。これはどういうことでしょうか？

  標準ライブラリには、何かをストリームに変換するためのトレイトがあります。`IntoStream`です。このトレイトには`into_stream`というメソッドがあり、`IntoStream`を実装したものをストリームに変換します。

  `std::iter::IntoIterator`とは異なり、`IntoStream`はまだコンパイラをサポートしていません。つまり、`for`ループのような自動変換はまだ起こらず、`into_stream`は常に手動で呼ばれなければなりません。

- Adapters

  ストリームを受け取って別のストリームを返す関数は、「アダプタパターン」の一形態であるため、しばしば「ストリームアダプタ」と呼ばれます。

  一般的なストリームアダプターには、[map](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#method.map)、[take](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#method.take)、[filter](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#method.filter)などがあります。詳しくは、それぞれのドキュメントをご覧ください。

- Laziness

  ストリーム（およびストリームアダプタ）は遅延しています。つまり、ストリームを作成しただけでは、何もしないということです。[next](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#tymethod.next)を呼び出すまで何も起こりません。これは、ストリームの副作用のためだけにストリームを作成する場合に、混乱の原因となることがあります。例えば、[map](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#method.map)メソッドは、反復する各要素に対してクロージャを呼び出します。

  ~~~rust
  let v = stream::repeat(1u8).take(5);
  v.map(|x| println!("{}", x));
  ~~~

  これは、ストリームを使用したのではなく、ストリームを作成しただけなので、値は表示されません。コンパイラはこのような動作について警告します。

  ~~~
  warning: unused result that must be used: streams are lazy and
  do nothing unless consumed
  ~~~

  副作用を考慮してマップを書くには、代わりに `while let`ループを使うのが一般的です。

  ~~~rust
  let mut v = stream::repeat(1u8).take(5);
  
  while let Some(x) = &v.next().await {
      println!("{}", x);
  }
  ~~~

  ストリームを評価する最も一般的な方法は、次のような`while let`ループを使う方法と、[collect](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#method.collect)メソッドを使って新しいコレクションを生成する方法です。

- Infinity

  ストリームは有限である必要はありません。例として、繰り返しのストリームは、無限のストリームです。

  ~~~rust
  let numbers = stream::repeat(1u8);
  ~~~

  無限のストリームを有限のストリームにするには、[take](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#method.take) streamアダプタを使用するのが一般的です。

  ~~~rust
  let numbers = stream::repeat(1u8);
  let mut five_numbers = numbers.take(5);
  
  while let Some(number) = five_numbers.next().await {
      println!("{}", number);
  }
  ~~~

  これにより、0 から 4 までの数字がそれぞれの行に表示されます。

  無限ストリームに対するメソッドは、数学的に有限時間で結果が得られるものであっても、終了しない場合があることに注意してください。特に、[min](https://docs.rs/async-std/1.9.0/async_std/stream/trait.Stream.html#method.min)などのメソッドは、一般的なケースでは、ストリームのすべての要素を走査する必要がありますが、無限のストリームでは正常に返されない可能性があります。

  ~~~rust
  let ones = async_std::stream::repeat(1);
  let least = ones.min().await.unwrap(); // Oh no! An infinite loop!
  // `ones.min()` causes an infinite loop, so we won't reach this point!
  println!("The smallest number one is {}.", least);
  ~~~

---

### [async_std::stream::interval](https://docs.rs/async-std/1.9.0/async_std/stream/fn.interval.html)

- Description

  設定された間隔で降伏する新しいストリームを作成します。

  このストリームは、`dur`後に最初に降伏し、その後はdurごとに降伏し続けます。ストリームは呼び出しの間に経過した時間を考慮し、時間のズレが生じないように適宜調整されます。

  各間隔は、指定された持続時間より若干長くなることがありますが、それ以下になることはありません。

  インターバルは高解像度のタイマーを意図したものではありませんが、むしろ、他の方法で起動するように指示された正確な瞬間の後に、ある程度の粒度で起動する可能性があることに注意してください。

  関連項目： [task::sleep](https://docs.rs/async-std/1.9.0/async_std/task/fn.sleep.html)

  ※不安定版のみサポート

- Example

  ~~~rust
  use async_std::prelude::*;
  use async_std::stream;
  use std::time::Duration;
  
  let mut interval = stream::interval(Duration::from_secs(4));
  while let Some(_) = interval.next().await {
      println!("prints every four seconds");
  }
  ~~~

  

  

