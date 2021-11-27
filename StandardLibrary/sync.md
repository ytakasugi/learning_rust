### std::sync

---

#### std::sync::mpsc

  - Description

    このモジュールは、3 つのタイプの中で具体的に定義されたチャネルを介したメッセージベースの通信を提供します。

    - 送信者

    - シンクロ送信者

    - 受信機

    `Sender`または`SyncSender`は、`Receiver`にデータを送信するために使用される。どちらの送信者もクローン可能（Multi-producer）で、多くのスレッドが同時に 1 つのレシーバに送信することができる（single-consumer）。

    これらのチャンネルには 2 つの種類があります。

    - 非同期で無限にバッファリングされたチャンネルです。チャンネル関数は (Sender, Receiver) タプルを返します。チャネルは、概念的には無限のバッファを持つ。

    - 同期的な、制限されたチャンネル。`sync_channel`関数は (SyncSender, Receiver) タプルを返す。バッファスペースが空くまでブロックすることで、すべての送信は同期的に行われる。`0`の束縛が許されているため、チャネルは "rendezvous" channel となり、各送信者がメッセージを受信者にアトム単位で渡すようになることに注意。

    Disconnection
    チャンネルの送受信操作はすべて、操作が成功したかどうかを示す結果を返します。操作が成功しなかった場合は、通常はチャンネルの残りの半分が対応するスレッドに落とされて「ハングアップ」したことを示している。
    チャネルの半分が割り当てられてしまうと、ほとんどの操作は進行を続けることができなくなるため、`Err`が返されます。多くのアプリケーションでは、このモジュールから返された結果をアンラップし続け、あるスレッドが予期せず死んでしまった場合には、スレッド間で失敗が伝播してしまう。

---

#### std::sync::mpsc::channel

  - Description

    新しい非同期チャンネルを作成し、送信者と受信者の半分を返す。`Sender`で送信されたすべてのデータは、送信された順に `Receiver`で利用可能になり、`send`が呼び出し元のスレッドをブロックすることはない(このチャンネルには "無限のバッファ" があり、バッファの限界に達するとブロックされる `sync_channel`とは異なる)。
    `Sender` をクローンして同じチャンネルに複数回送信することができますが、サポートされているのは 1 つの`Receiver`のみ。

    `Sender`で送信しようとしている間に`Receiver`が切断された場合、送信メソッドは `SendError`を返す。同様に、`Sender`が`recv`しようとしているときに切断された場合、`recv`メソッドは`RecvError`を返す。

---

#### std::sync::mpsc::Sender::send

  - Description

    このチャネルに値を送信しようとし、送信できなかった場合は値を返す。
    送信が成功した場合は、チャンネルの相手側ハングアップしていないと判断された場合。送信に失敗した場合は、対応するチャンネルが既に割り当て解除されている場合。`Err`の戻り値はデータを受信しないことを意味し、`Ok`の戻り値はデータを受信することを意味しないことに注意すること。この関数が`Ok`を返した直後に、対応するチャンネルがハングアップする可能性がある。
    このメソッドは、現在のスレッドをブロックすることはない。

---

#### std::sync::mpsc::Receive::recv

  - Description

    この受信機で値の待ち受けを試み、対応するチャンネルがハングアップした場合はエラーを返す。
    この関数は、利用可能なデータがなく、より多くのデータを送信できる可能性がある場合、常に現在のスレッドをブロックする。対応する`Sender`(または`SyncSender`) にメッセージが送信されると、このレシーバはウェイクアップしてそのメッセージを返す。
    対応する`Sender`が切断された場合や、このコールがブロックされている間に切断された場合は、このコールはウェイクアップして`Err`を返し、このチャンネルではこれ以上メッセージを受信できないことを示す。ただし、チャネルはバッファリングされているので、切断前に送信されたメッセージは正しく受信される。

---

#### std::sync::Mutex

  - Description

  共有データの保護に有用な相互排除プリミティブ
  このmutexは、ロックが利用可能になるのを待つスレッドをブロックする。`mutex`は静的に初期化したり、新しいコンストラクタを使って作成することもできます。各`mutex`には保護するデータを表す`type`パラメータがあります。データは`lock`と`try_lock`から返される RAII ガードを介してのみアクセスでき、`mutex`がロックされているときにのみデータにアクセスできることを保証する。

  - Poisoning
    このモジュールのmutexは「Poisoning」と呼ばれる戦略を実装しており、mutexを保持している間にスレッドがパニックになると、いつでもmutexがポイズニングされているとみなされます。一度mutexがポイズニングされると、他のすべてのスレッドはデータが汚染されている可能性が高いので、デフォルトではデータにアクセスできなくなります(何らかの不変量が保持されていない)。
    mutexの場合、これは`lock`メソッドと`try_lock`メソッドが、`mutex`がポイズンされたかどうかを示す `Result`を返すことを意味します。mutexのほとんどの使用法では、これらの結果を単に unwrap() して、無効な不変量が目撃されないようにスレッド間でパニックを伝播させる。
    しかし、ポイズンされたmutexは、基礎となるデータへのすべてのアクセスを妨げるものではない。`PoisonError`型には`into_inner`メソッドがあり、これはロックが成功したときに返されるはずのガードを返す。これにより、ロックがポイズンされているにもかかわらず、データへのアクセスが可能になる。



---

#### std::sync::Mutex::lock

  - Description

    mutexを取得し、それが可能になるまで現在のスレッドをブロックします。

    この関数は、mutexを取得できるようになるまでローカルスレッドをブロックします。解放時には、そのスレッドはロックが保持されている唯一のスレッドとなります。ロックのスコープ付きアンロックを可能にするために、`RAII`ガードが返されます。ガードがスコープ外になると、mutexはアンロックされる。
    既にロックを保持しているスレッドでmutexをロックする場合の正確な動作は未定義である。しかし、この関数は2回目の呼び出しでは戻りません(例えば、パニックやデッドロックになる可能性がある)。

    - Error
      このmutexを保持している間にこのmutexの他のユーザがパニックに陥った場合、この呼び出しはmutexを取得した後にエラーを返す。
    - Panic
      この関数は、現在のスレッドが既にロックを保持している場合に呼び出されるとパニックになる可能性がある。

---

#### std::sync::Arc

  - Description

    スレッドセーフな参照カウントポインタ。`Arc`は`Atomically Reference Counted`の略。

    `Arc<T>`型は、ヒープに割り当てられた`T`型の値の共有所有権を提供する。`Arc`上で`clone`を実行すると、参照カウントを増加させながら、ソース`Arc`と同じヒープ上の割り当てを指す新しい`Arc`インスタンスが生成される。与えられたアロケーションへの最後の`Arc`ポインタが破棄されると、そのアロケーションに格納されている値 (多くの場合、「内部値」と呼ばれます) も削除されます。

    Rust の共有参照はデフォルトで突然変更されることを禁止しており、`Arc`も例外ではありません。`Arc`を通してミューテーションを行う必要がある場合は、`Mutex`、`RwLock`、または`Atomic`型のいずれかを使用してください。

    `Rc<T>`とは異なり、`Arc<T>`は参照カウントにアトミック演算を使用します。これはスレッドセーフであることを意味します。欠点は、アトミック演算が通常のメモリアクセスに比べて高価なことです。スレッド間で参照カウントされた割り当てを共有しない場合は、より低いオーバーヘッドのために`Rc<T>`の使用を検討してください。スレッド間で`Rc<T> `を送ろうとすると、コンパイラがそれをキャッチするので、`Rc<T>`は安全なデフォルトです。しかし、ライブラリの利用者に柔軟性を持たせるために、ライブラリは`Arc<T>`を選択するかもしれません。

    `Arc<T>`は、`T`が`Send`と`Sync`を実装している限り、`Send`と`Sync`をする。スレッドセーフではない型の`T`を`Arc<T>`に入れてスレッドセーフにすることができないのはなぜか？最初は少し直観的ではないかもしれないが、結局のところ、`Arc<T>` のスレッドセーフは重要ではないのではないのか？結局のところ、`Arc<T>`のスレッド安全性は重要ではないのではないのか？重要なのは、`Arc<T>`は、同じデータの複数の所有権を持つことをスレッドセーフにする、そのデータにスレッドセーフを追加するわけではない。`Arc<RefCell<T>>`を考えてみる。`RefCell<T>`は`Sync`ではないので、もし`Arc<T>`が常に`Send`であれば、`Arc<RefCell<T>`も同様に`Send`になります。しかし、そうすると問題が発生する。`RefCell<T>`はスレッドセーフではない。

    `RefCell<T>`はスレッドセーフではないので、非アトミック演算を使って借用回数を追跡する。

    `downgrade`メソッドを使用して、所有権のない`Weak`ポインタを作成することができる。`Weak`ポインタを`Arc`にアップグレードすることができますが、アロケーションに格納されている値が既にドロップされている場合は`None`を返す。言い換えれば、`Weak` ポインタはアロケーション内の値を保持しませんが、アロケーション (値の裏付けとなるストア) を保持する。

    `Arc`ポインタ間のサイクルは決して解放されない。このため、`Weak`はサイクルを壊すために使用されます。例えば、ツリーは親ノードから子ノードへの強いアークポインタを持ち、子ノードから親ノードへの弱いポインタを持つことができる。

    - Cloning
      既存の参照カウントされたポインタから新しい参照を作成するには、`Arc<T>`と`Weak<T>`に実装された`Clone`トレイトを使用します。

~~~rust
    use std::sync::Arc;
    let foo = Arc::new(vec![1.0, 2.0, 3.0]);
    // The two syntaxes below are equivalent.
    let a = foo.clone();
    let b = Arc::clone(&foo);
    // a, b, and foo are all Arcs that point to the same memory location
~~~

    - Deref behavior
      `Arc<T>`は自動的に (`Deref`トレイトを介して) `T`に派生するので、`Arc<T>`型の値に対して`T`のメソッドを呼び出すことができる。`T`のメソッドとの名前の衝突を避けるため、`Arc<T>`のメソッドは関連する関数であり、完全修飾構文を用いて呼び出される。

~~~rust
    use std::sync::Arc;
    
    let my_arc = Arc::new(());
    Arc::downgrade(&my_arc);
~~~

    `Clone` のようなトレイトの`Arc<T>`の実装も、完全修飾構文を使って呼ばれることがある。

~~~rust
    use std::sync::Arc;
    
    let arc = Arc::new(());
    // Method-call syntax
    let arc2 = arc.clone();
    // Fully qualified syntax
    let arc3 = Arc::clone(&arc);
~~~

    `Weak<T>`は、内部の値が既にドロップされている可能性があるため、`T`への自動参照は行わない。

---

#### std::sync::Arc::new

- Description

  新しい`Arc<T>`を構築します。

- Example

```rust
  use std::sync::Arc;
  
  let five = Arc::new(5);
```



---

#### std::sync::Arc::clone

- Description

  `Arc`ポインタのクローンを作成します。

  これにより、同じアロケーションへの別のポインタが作成され、強い参照カウントが増加します。

- Example

```rust
  use std::sync::Arc;
  
  let five = Arc::new(5);
  
  let _ = Arc::clone(&five);
```



---

#### std::sync::RwLock

- Description

  リーダライタロック

  このタイプのロックでは、任意の時点で複数のリーダーまたは最大1つのライターを許可します。このロックの書き込み部分は、通常、基礎となるデータの変更を可能にし（排他的アクセス）、このロックの読み取り部分は、通常、読み取り専用のアクセスを可能にします（共有アクセス）。

  一方、`Mutex`は、ロックを取得するリーダーとライターを区別しないため、ロックが利用可能になるのを待つスレッドがブロックされます。`RwLock`は、ライターがロックを保持していない限り、何人のリーダーでもロックを取得することができます。

  ロックの優先ポリシーは、基礎となるオペレーティングシステムの実装に依存しており、このタイプは特定のポリシーが使用されることを保証するものではありません。特に、書き込み時にロックの取得を待っているライターは、同時に行われる読み取りの呼び出しをブロックする場合もありますし、しない場合もあります。

  型パラメータTは、このロックが保護するデータを表します。`T`は、スレッド間で共有するための`Send`と、リーダーによる同時アクセスを可能にするための`Sync`を満たすことが要求されます。ロックメソッドから返される RAII ガードは、ロックの内容へのアクセスを許可するために `Deref `(および`write`メソッドの `DerefMut`) を実装しています。

- Poisoning

  `RwLock`は、`Mutex`と同様に、パニックが発生するとポイズンになります。ただし、`RwLock`がポイズンされるのは、それが排他的にロックされている間（書き込みモード）にパニックが発生した場合のみであることに注意してください。パニックがリーダーで発生した場合は、ロックはポイズニングされません。

- Example

```rust
  use std::sync::RwLock;
  
  let lock = RwLock::new(5);
  
  // many reader locks can be held at once
  {
      let r1 = lock.read().unwrap();
      let r2 = lock.read().unwrap();
      assert_eq!(*r1, 5);
      assert_eq!(*r2, 5);
  } // read locks are dropped at this point
  
  // only one write lock may be held, however
  {
      let mut w = lock.write().unwrap();
      *w += 1;
      assert_eq!(*w, 6);
  } // write lock is dropped here
```




---

#### std::sync::RwLock::new

- Description

  ロックが解除された`RwLock<T>`の新しいインスタンスを作成します。

- Example

```rust
  use std::sync::RwLock;
  
  let lock = RwLock::new(5);
```



---

#### std::sync::RwLock::read

- Description

  `rwlock`を共有の読み取りアクセスでロックし、それを取得できるまで現在のスレッドをブロックします。

  呼び出したスレッドは、ロックを保持するライターが無くなるまでブロックされます。このメソッドが戻るときには、現在ロックの内側に他のリーダーがいるかもしれません。このメソッドは、競合するリーダとライタのどちらが先にロックを取得するかという順序に関して、いかなる保証も行いません。

  このスレッドの共有アクセスが削除されると、それを解放する`RAII`ガードを返します。

- Errors

  この関数は、`RwLock`がポイズンされた場合、エラーを返します。`RwLock`は、排他的ロックを保持している間にライターがパニックを起こすと、ポイズンされます。ロックを取得した直後に障害が発生します。

- Panics

  この関数は、ロックがすでに現在のスレッドによって保持されている場合、呼び出されるとパニックになることがあります。

- Example

```rust
  use std::sync::{Arc, RwLock};
  use std::thread;
  
  let lock = Arc::new(RwLock::new(1));
  let c_lock = Arc::clone(&lock);
  
  let n = lock.read().unwrap();
  assert_eq!(*n, 1);
  
  thread::spawn(move || {
      let r = c_lock.read();
      assert!(r.is_ok());
  }).join().unwrap();
```



---

#### std::sync::RwLock::write

- Description

  `rwlock`を排他的な書き込みアクセスでロックし、それが取得できるまで現在のスレッドをブロックします。

  この関数は、他のライターまたは他のリーダーが現在ロックにアクセスしている間は戻りません。

  ドロップされたときにこの`rwlock`の書き込みアクセスをドロップする`RAII`ガードを返します。

- Errors

  この関数は、`RwLock`がポイズンされた場合、エラーを返します。`RwLock`は、ライターが排他的ロックを保持している間にパニックになると、ポイズンされます。ロックを取得するとエラーが返されます。

- Panics

  この関数は、ロックがすでに現在のスレッドによって保持されている場合、呼び出されるとパニックになることがあります。

- Example

```rust
  use std::sync::RwLock;
  
  let lock = RwLock::new(1);
  
  let mut n = lock.write().unwrap();
  *n = 2;
  
  assert!(lock.try_read().is_err());
```



---

#### std::sync::atomic

- Description

  Atomic型

  アトミック型はスレッド間のプリミティブな共有メモリ通信を提供し、他の並行型の構成要素となります。
  このモジュールは、`AtomicBool`、`AtomicIsize`、`AtomicUsize`、`AtomicI8`、`AtomicU16`などを含む、選択された数のAtomic型のアトミックバージョンを定義します。Atomic型は、正しく使用されるとスレッド間の更新を同期させる操作を提供します。
  各メソッドは、その操作のためのメモリバリアの強さを表す順序を取ります。これらの順序付けは、C++20 のアトミック順序付けと同じです。詳細については、[nomicon](https://doc.rust-lang.org/stable/nomicon/atomics.html)を参照してください。
  Atomic変数はスレッド間で共有しても安全ですが（`Sync`を実装しています）、それ自体は共有のメカニズムを提供しておらず、Rustのスレッドモデルに従っています。アトミック変数を共有する最も一般的な方法は、`Arc`(原子的に参照カウントされた共有ポインタ) に格納することです。
  Atomic型は静的変数に格納され、`AtomicBool::new`のような定数初期化子を使って初期化されます。Atomic静的変数は、遅延グローバル初期化によく使われます。

---

#### std::sync::Condvar

- Description

  コンディション変数

  コンディション変数とは、あるイベントの発生を待つ間、CPU時間を消費しないようにスレッドをブロックする機能です。コンディション変数は通常、ブーリアン値の述語（コンディション）とミューテックスに関連付けられています。述語は、スレッドをブロックしなければならないと判断する前に、常にミューテックスの内部で検証されます。

  このモジュールの関数は、現在の実行スレッドをブロックします。同じ条件変数に複数のミューテックスを使おうとすると、ランタイム・パニックを起こす可能性があることに注意してください。

- Example

```rust
  use std::sync::{Arc, Mutex, Condvar};
  use std::thread;
  
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair2 = Arc::clone(&pair);
  
  // Inside of our lock, spawn a new thread, and then wait for it to start.
  thread::spawn(move|| {
      let (lock, cvar) = &*pair2;
      let mut started = lock.lock().unwrap();
      *started = true;
      // We notify the condvar that the value has changed.
      cvar.notify_one();
  });
  
  // Wait for the thread to start up.
  let (lock, cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  while !*started {
      started = cvar.wait(started).unwrap();
  }
```



---

#### std::sync::Condvar::wait

- Description

  この条件変数が通知を受け取るまで、現在のスレッドをブロックします。

  この関数は、指定された（guardで表される）ミューテックスをアトミックにアンロックし、現在のスレッドをブロックします。これは、ミューテックスのロックが解除された後に論理的に発生する`notify_one`や`notify_all`の呼び出しが、このスレッドを起こす候補になることを意味します。この関数呼び出しが戻るとき、指定されたロックは再取得されています。

  この関数は、偽のウェイクアップの影響を受けやすいことに注意してください。条件変数には通常、ブーリアン値の述語が関連付けられており、偽のウェイクアップから守るために、この関数が戻るたびに述語をチェックしなければなりません。

- Example

```rust
  use std::sync::{Arc, Mutex, Condvar};
  use std::thread;
  
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair2 = Arc::clone(&pair);
  
  thread::spawn(move|| {
      let (lock, cvar) = &*pair2;
      let mut started = lock.lock().unwrap();
      *started = true;
      // We notify the condvar that the value has changed.
      cvar.notify_one();
  });
  
  // Wait for the thread to start up.
  let (lock, cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  // As long as the value inside the `Mutex<bool>` is `false`, we wait.
  while !*started {
      started = cvar.wait(started).unwrap();
  }
```



---

#### std::sync::Condvar::notify_all

- Description

  この条件変数でブロックされているすべてのスレッドを起動させます。

  このメソッドは、条件変数の現在の待機者がすべて起こされることを保証します。`notify_all()`の呼び出しは、いかなる方法でもバッファリングされません。

  1つのスレッドだけを目覚めさせるには、`notify_one`を参照してください。

- Example

```rust
  use std::sync::{Arc, Mutex, Condvar};
  use std::thread;
  
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  let pair2 = Arc::clone(&pair);
  
  thread::spawn(move|| {
      let (lock, cvar) = &*pair2;
      let mut started = lock.lock().unwrap();
      *started = true;
      // We notify the condvar that the value has changed.
      cvar.notify_all();
  });
  
  // Wait for the thread to start up.
  let (lock, cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  // As long as the value inside the `Mutex<bool>` is `false`, we wait.
  while !*started {
      started = cvar.wait(started).unwrap();
  }
```

---

#### std::sync::Barrier

- Description

  バリアは、複数のスレッドがある計算の開始を同期させるためのものです。

- Example

```rust
  use std::sync::{Arc, Barrier};
  use std::thread;
  
  let mut handles = Vec::with_capacity(10);
  let barrier = Arc::new(Barrier::new(10));
  for _ in 0..10 {
      let c = Arc::clone(&barrier);
      // The same messages will be printed together.
      // You will NOT see any interleaving.
      handles.push(thread::spawn(move|| {
          println!("before wait");
          c.wait();
          println!("after wait");
      }));
  }
  // Wait for other threads to finish.
  for handle in handles {
      handle.join().unwrap();
  }
```

---

#### std::sync::Barrier::wait

- Description

  すべてのスレッドがここでランデブーするまで、現在のスレッドをブロックします。

  バリアは、すべてのスレッドが一度ランデブーした後も再利用可能で、継続的に使用することができます。

  単一の（任意の）スレッドは、この関数から戻るときに`BarrierWaitResult::is_leader()`から`true`を返す `BarrierWaitResult`を受け取り、他のすべてのスレッドは`BarrierWaitResult::is_leader()`から`false`を返す結果を受け取ることになります。

- Example

```rust
  use std::sync::{Arc, Barrier};
  use std::thread;
  
  let mut handles = Vec::with_capacity(10);
  let barrier = Arc::new(Barrier::new(10));
  for _ in 0..10 {
      let c = Arc::clone(&barrier);
      // The same messages will be printed together.
      // You will NOT see any interleaving.
      handles.push(thread::spawn(move|| {
          println!("before wait");
          c.wait();
          println!("after wait");
      }));
  }
  // Wait for other threads to finish.
  for handle in handles {
      handle.join().unwrap();
  }
```




---

#### std::sync::atomic::AtomicUsize

- Description

  スレッド間で安全に共有することができる整数型。

  この型は、基礎となる整数型である`usize`と同じメモリ内表現を持ちます。アトミック型と非アトミック型の違いや、この型の移植性については、モジュールレベルのドキュメントを参照してください。

  注意：この型は、`usize`のアトミックなロードとストアをサポートするプラットフォームでのみ使用できます。

- Implementations

  - fetch_add

    - Description

      現在の値に加算し、前の値を返します。

      この操作は、オーバーフロー時に折り返します。

      `fetch_add`は、この操作のメモリ順序を記述する`Ordering`引数を取ります。すべての順序モードが可能です。`Acquire`を使用すると、この操作のストア部分がRelaxedになり、`Release`を使用するとロード部分が`Relaxed`になることに注意してください。

      注：このメソッドは、`usize`のアトミック操作をサポートするプラットフォームでのみ使用できます。

    - Example

    ```rust
      use std::sync::atomic::{AtomicUsize, Ordering};
      
      let foo = AtomicUsize::new(0);
      assert_eq!(foo.fetch_add(10, Ordering::SeqCst), 0);
      assert_eq!(foo.load(Ordering::SeqCst), 10);
    ```

  - fetch_sub

    - Description

      現在の値から減算し、前の値を返します。

      この操作は、オーバーフロー時に折り返します。

      `fetch_sub`は、この操作のメモリ順序を記述する`Ordering`引数を取ります。すべての順序モードが可能です。`Acquire`を使用すると、この操作のストア部分が`Relaxed`になり、`Release`を使用するとロード部分が`Relaxed`になることに注意してください。

      注：このメソッドは、`usize`のアトミック操作をサポートするプラットフォームでのみ使用できます。

    - Example

    ```rust
      use std::sync::atomic::{AtomicUsize, Ordering};
      
      let foo = AtomicUsize::new(20);
      assert_eq!(foo.fetch_sub(10, Ordering::SeqCst), 20);
      assert_eq!(foo.load(Ordering::SeqCst), 10);
    ```

  - load

    - Description

      Atomicな整数から値をロードします。

      `load`は、この操作のメモリ順序を記述する`Ordering`引数を取ります。可能な値は、`SeqCst`、`Acquire`、`Relaxed`です。

    - panics

      orderがが`Release`や`AcqRel`の場合はパニックになります。

    - Example

    ```rust
      use std::sync::atomic::{AtomicUsize, Ordering};
      
      let some_var = AtomicUsize::new(5);
      
      assert_eq!(some_var.load(Ordering::Relaxed), 5);
    ```

---

#### std::sync::atomic::Ordering

- Description

  アトミックメモリの順序付け

  メモリの順序付けは、原子演算がメモリを同期させる方法を指定します。最も弱い`Ordering::Relaxed`では、操作によって直接触れられたメモリだけが同期される。一方、`Ordering::SeqCst`のストア・ロードのペアは、他のメモリを同期させ、さらにすべてのスレッドでそのような操作の合計順序を保持します。

  Rust のメモリ順序は、C++20のものと同じです。

  詳細は[nomicon](https://doc.rust-lang.org/nomicon/atomics.html)を参照してください。

- Variants 

  - Relaxed
    順序の制約はなく、アトミックな操作のみ。

    C++20 の`memory_order_relaxed`に相当します。

  - Release

    ストアと組み合わせた場合、以前のすべての操作は、`Acquire`（またはそれ以上）の順序でこの値のロードの前に順序付けられます。特に、以前の書き込みはすべて、この値の`Acquire`（またはそれ以上）のロードを実行するすべてのスレッドから見えるようになります。

    ロードとストアを組み合わせた操作にこの順序を使用すると、`Relaxed load`操作になることに注意してください。

    この順序は、ストアを実行できる操作にのみ適用されます。

    C++20 では`memory_order_release`に対応しています。

  - Acquire

    ロードと組み合わせた場合、ロードされた値が`Release`（またはより強い）順序のストア操作によって書き込まれた場合、後続のすべての操作はそのストアの後に順序付けられるようになります。特に、後続のロードはすべて、ストアの前に書き込まれたデータを見ることになります。

    ロードとストアを組み合わせた操作にこの順序を使用すると、`Relaxed store`操作になることに注意してください。

    この順序は、ロードを行うことができる操作にのみ適用されます。

    C++20 の`memory_order_acquire`に対応しています。

  - AcqRel

    `Acquire`と`Release`の両方の効果を併せ持つ。ロードでは`Acquire`順序を使用し、ストアでは`Release`順序を使用します。

    `compare_and_swap`の場合は、操作がストアを実行せずに終わる可能性があるため、Acquire 順序だけになっていることに注意してください。しかし、`AcqRel`が`Relaxed`アクセスを実行することはありません。

    この順序は、ロードとストアの両方を組み合わせた操作にのみ適用されます。

    C++20 の`memory_order_acq_rel`に対応しています。

  - SeqCst

    `Acquire`/`Release`/`AcqRel`（それぞれロード、ストア、ストア付きロードの操作）に、すべてのスレッドがすべての連続した操作を同じ順序で見ることができるという保証を追加したもの。

    C++20 の`memory_order_seq_cst`に対応しています。

