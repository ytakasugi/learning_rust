### std::mem

- Description

  メモリを扱うための基本的な関数です。

  このモジュールには、型のサイズや配置の問い合わせ、メモリの初期化や操作のための関数が含まれています。

---

#### std::mem::size_of

- Description

  型のサイズをバイト単位で返します。

  具体的には、そのアイテムタイプを持つ配列の連続する要素間のオフセットをバイト単位で表します（アライメントパディングを含む）。したがって、任意の型Tと長さnに対して、`[T; n]`は`n * size_of::<T>()`のサイズを持ちます。

  一般に、型のサイズはコンパイル時に安定しませんが、プリミティブなどの特定の型は安定します。

  次の表は、プリミティブのサイズを示しています。

  | Type | size_of::<Type>() |
  | :--- | :---------------- |
  | ()   | 0                 |
  | bool | 1                 |
  | u8   | 1                 |
  | u16  | 2                 |
  | u32  | 4                 |
  | u64  | 8                 |
  | u128 | 16                |
  | i8   | 1                 |
  | i16  | 2                 |
  | i32  | 4                 |
  | i64  | 8                 |
  | i128 | 16                |
  | f32  | 4                 |
  | f64  | 8                 |
  | char | 4                 |

  さらに、`usize`と`isize`は同じサイズです。

  型`*const T`, `&T`, `Box<T>`, `Option<&T>`,` Option<Box<T>> `はすべて同じサイズを持っています。`T`が`Sized`の場合、それらの型はすべて`usize`と同じサイズを持ちます。

  ポインタの`mutability`はそのサイズを変えません。そのため、`&T`と`&mut T`は同じサイズになります。同様に、`*const T`と`*mut T`も同じです。

- Size of `#[repr(C)]` items

  アイテムのC表現は、レイアウトが定義されています。このレイアウトでは、すべてのフィールドのサイズが安定していれば、アイテムのサイズも安定しています。

- Size of Structs

  構造体の場合は、以下のアルゴリズムでサイズを決定します。

  構造体の各フィールドを宣言順に並べる。

  - フィールドのサイズを追加する。

  - 現在のサイズを切り上げて、次のフィールドのアラインメントの最も近い倍数にします。

  最後に、構造体のサイズをそのアラインメントの最も近い倍数に丸めます。構造体のアラインメントは、通常、構造体のすべてのフィールドのうち最大のアラインメントとなります。

  Cとは異なり、ゼロサイズの構造体は1バイトに切り上げられません。
  
- Size of Enums

  判別子以外のデータを持たない列挙型は、コンパイルされたプラットフォームのC列挙型と同じサイズになります。

- Size of Unions

  共用体のサイズは、その最大のフィールドのサイズです。

  C言語とは異なり、サイズがゼロの共用体は1バイトに切り上げられません。

- Example

```rust
  use std::mem;
  
  // Some primitives
  assert_eq!(4, mem::size_of::<i32>());
  assert_eq!(8, mem::size_of::<f64>());
  assert_eq!(0, mem::size_of::<()>());
  
  // Some arrays
  assert_eq!(8, mem::size_of::<[i32; 2]>());
  assert_eq!(12, mem::size_of::<[i32; 3]>());
  assert_eq!(0, mem::size_of::<[i32; 0]>());
  
  
  // Pointer size equality
  assert_eq!(mem::size_of::<&i32>(), mem::size_of::<*const i32>());
  assert_eq!(mem::size_of::<&i32>(), mem::size_of::<Box<i32>>());
  assert_eq!(mem::size_of::<&i32>(), mem::size_of::<Option<&i32>>());
  assert_eq!(mem::size_of::<Box<i32>>(), mem::size_of::<Option<Box<i32>>>());
```

  Using `#[repr(C)]`.

```rust
  use std::mem;
  
  #[repr(C)]
  struct FieldStruct {
      first: u8,
      second: u16,
      third: u8
  }
  
  // The size of the first field is 1, so add 1 to the size. Size is 1.
  // The alignment of the second field is 2, so add 1 to the size for padding. Size is 2.
  // The size of the second field is 2, so add 2 to the size. Size is 4.
  // The alignment of the third field is 1, so add 0 to the size for padding. Size is 4.
  // The size of the third field is 1, so add 1 to the size. Size is 5.
  // Finally, the alignment of the struct is 2 (because the largest alignment amongst its
  // fields is 2), so add 1 to the size for padding. Size is 6.
  assert_eq!(6, mem::size_of::<FieldStruct>());
  
  #[repr(C)]
  struct TupleStruct(u8, u16, u8);
  
  // Tuple structs follow the same rules.
  assert_eq!(6, mem::size_of::<TupleStruct>());
  
  // Note that reordering the fields can lower the size. We can remove both padding bytes
  // by putting `third` before `second`.
  #[repr(C)]
  struct FieldStructOptimized {
      first: u8,
      third: u8,
      second: u16
  }
  
  assert_eq!(4, mem::size_of::<FieldStructOptimized>());
  
  // Union size is the size of the largest field.
  #[repr(C)]
  union ExampleUnion {
      smaller: u8,
      larger: u16
  }
  
  assert_eq!(2, mem::size_of::<ExampleUnion>());
```



---

#### std::mem::size_of_val

- Description

  指し示された値のサイズをバイト単位で返します。

  これは通常`size_of::<T>()`と同じです。しかし、スライス`[T]`やトレイトオブジェクトなど、`T`が静的に知られているサイズを持たない場合は、 `size_of_val`を使って動的に知られているサイズを得ることができます。

- Example

```rust
  use std::mem;
  
  assert_eq!(4, mem::size_of_val(&5i32));
  
  let x: [u8; 13] = [0; 13];
  let y: &[u8] = &x;
  assert_eq!(13, mem::size_of_val(y));
```

---

#### std::mem::size_of_val_raw

- Description

  指し示された値のサイズをバイト単位で返します。

  これは通常`size_of::<T>()`と同じです。しかし、スライス`T`やトレイトオブジェクトなど、`T`が静的に知られているサイズを持たない場合は、 `size_of_val_raw`を使って動的に知られているサイズを得ることができます。

- Safety

  この関数は、以下の条件が成立する場合にのみ、安全に呼び出すことができます。

  - `T`が`Sized`の場合、この関数は常に安全に呼び出すことができます。
  - もし、`T`が`Sized`出ない場合・・・
    - スライスの場合、スライスのテールの長さは初期化された整数でなければならず、値全体のサイズ（動的なテールの長さ＋静的なサイズのプレフィックス）は`isize`に収まらなければなりません。
    - トレイトオブジェクトの場合、ポインタの`vtable`部分は、サイズ変更の強制によって得られた有効な`vtable`を指していなければならず、値全体のサイズ（動的なテールの長さ＋静的サイズのプレフィックス）が`isize`に収まっていなければなりません
    - （不安定な）外部型である場合，この関数は常に安全に呼び出すことができますが，外部型のレイアウトがわからないため，パニックを起こしたり，誤った値を返したりする可能性があります。これは、`extern`型の末尾を持つ型への参照に対する`size_of_val`と同じ動作です。
    - そうでない場合は、保守的にこの関数を呼び出すことはできません。

- Example

```rust
  #![feature(layout_for_ptr)]
  use std::mem;
  
  assert_eq!(4, mem::size_of_val(&5i32));
  
  let x: [u8; 13] = [0; 13];
  let y: &[u8] = &x;
  assert_eq!(13, unsafe { mem::size_of_val_raw(y) });
```

---

#### std::mem::discriminant

- Description

  vのenum variantを一意に識別する値を返します。

  `T`がenumでない場合、この関数を呼び出しても未定義の動作にはなりませんが、戻り値は不定です。

- Stability

  enumの定義が変更された場合、enumのvariantの判別式は変更される可能性があります。あるバリアントの判別式は、同じコンパイラでコンパイルしても変わりません。

- Example

  これは、実際のデータを無視して、データを運ぶenumを比較するために使用することができます。

```rust
  use std::mem;
  
  enum Foo { A(&'static str), B(i32), C(i32) }
  
  assert_eq!(mem::discriminant(&Foo::A("bar")), mem::discriminant(&Foo::A("baz")));
  assert_eq!(mem::discriminant(&Foo::B(1)), mem::discriminant(&Foo::B(2)));
  assert_ne!(mem::discriminant(&Foo::B(3)), mem::discriminant(&Foo::C(3)));
```

---

#### std::mem::replace

- Description

  `src`を参照されている`dest`に移動させ、前の`dest`の値を返します。

  どちらの値も破棄しません。

  - 2つの変数の値を入れ替えたい場合は、[`swap`](https://doc.rust-lang.org/std/mem/fn.swap.html)を参照してください。
  - デフォルト値に置き換えたい場合は、[`take`](https://doc.rust-lang.org/std/mem/fn.take.html)をご覧ください。

- Example

```rust
  use std::mem;
  
  let mut v: Vec<i32> = vec![1, 2];
  
  let old_v = mem::replace(&mut v, vec![3, 4, 5]);
  assert_eq!(vec![1, 2], old_v);
  assert_eq!(vec![3, 4, 5], v);
```

  `replace`は、構造体のフィールドを別の値に置き換えて消費することができます。置換を行わないと、以下のような問題が発生します。

```rust
  struct Buffer<T> { buf: Vec<T> }
  
  impl<T> Buffer<T> {
      fn replace_index(&mut self, i: usize, v: T) -> T {
          // error: cannot move out of dereference of `&mut`-pointer
          let t = self.buf[i];
          self.buf[i] = v;
          t
      }
  }
```

  `T`は必ずしも`Clone`を実装しているわけではないので、ムーブを避けるために`self.buf[i]`を`clone`することもできないことに注意してください。しかし、`replace`を使えば、そのインデックスにある元の値を`self`から切り離し、それを返すことができます。

```rust
  use std::mem;
  
  impl<T> Buffer<T> {
      fn replace_index(&mut self, i: usize, v: T) -> T {
          mem::replace(&mut self.buf[i], v)
      }
  }
  
  let mut buffer = Buffer { buf: vec![0, 1] };
  assert_eq!(buffer.buf[0], 0);
  
  assert_eq!(buffer.replace_index(0, 2), 0);
  assert_eq!(buffer.buf[0], 2);
```
