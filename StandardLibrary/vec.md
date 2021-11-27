td::vec

---
#### std::vec::IntoIter

- Description

  ベクトルの外に移動するイテレータ。

  この構造体は、`[Vec]`の`into_iter`メソッドによって作成されます（`[IntoIterator] trait`によって提供されます）。

---

#### std::vec::Vec::extend_from_slice

- Description

  スライス内のすべての要素をクローン化し、`Vec`に追加します。

  他のスライスを反復処理し、各要素をクローン化し、この`Vec`に追加します。他のベクトルは順番に巡回されます．

  この関数は`extend`と同じですが、代わりにスライスを扱うように特化されていることに注意してください。Rustが特殊化された場合、この関数はおそらく非推奨になるでしょう（しかし、まだ利用可能です）。

- Example

~~~rust
  let mut vec = vec![1];
  vec.extend_from_slice(&[2, 3, 4]);
  assert_eq!(vec, [1, 2, 3, 4]);
~~~

---

#### std::vec::Vec::dedup

- Description

  [PartialEq](https://doc.rust-lang.org/stable/std/cmp/trait.PartialEq.html)トレイトの実装に従って，ベクトル中の連続した繰り返し要素を削除します。

  `Vec`がソートされている場合は，すべての重複した要素を削除します。

- Example

```rust
  let mut vec = vec![1, 2, 2, 3, 2];
  
  vec.dedup();
  
  assert_eq!(vec, [1, 2, 3, 2]);
```



---

#### std::vec::Vec::with_capacity

- Description

  指定された容量の新しい空の`Vec<T>`を作成します。

  `Vec`は、再割り当てを行わずに正確に容量要素を保持することができます。capacityが0の場合、ベクタは割り当てを行いません。

  返されたベクタは指定された容量を持っていますが、ベクタの長さはゼロになることに注意することが重要です。長さと容量の違いについての説明は、容量と再割り当てを参照してください。



---

#### std::vec::Vec::into_boxed_slice

- Description

  Vectorを`Box<[T]>`に変換します。

  これにより、余った容量がなくなることに注意してください。

- Example

```rust
  let v = vec![1, 2, 3];
  
  let slice = v.into_boxed_slice();
```

  Any excess capacity is removed:

```rust
  let mut vec = Vec::with_capacity(10);
  vec.extend([1, 2, 3].iter().cloned());
  
  assert_eq!(vec.capacity(), 10);
  let slice = vec.into_boxed_slice();
  assert_eq!(slice.into_vec().capacity(), 3);
```




---

#### std::vec::Vec::resize_with

- Description

  `len`が`new_len`と等しくなるように、`Vec`のサイズをその場で変更します。

  `new_len`が`len`よりも大きい場合、`Vec`はその差だけ拡張され、追加された各スロットはクロージャ`f`を呼び出した結果で埋められます。

  `new_len`が`len`よりも小さい場合、`Vec`は単に切り捨てられます。

  このメソッドはクロージャを使って、プッシュのたびに新しい値を生成します。与えられた値のクローンを作りたい場合は、`Vec::resize`を使います。`Default`トレイトを使って値を生成したい場合には、2 番目の引数に`Default::default`を渡します。

- Example

```rust
  let mut vec = vec![1, 2, 3];
  vec.resize_with(5, Default::default);
  assert_eq!(vec, [1, 2, 3, 0, 0]);
  
  let mut vec = vec![];
  let mut p = 1;
  vec.resize_with(4, || { p *= 2; p });
  assert_eq!(vec, [2, 4, 8, 16]);
```



---

#### std::vec::Vec::insert

- Description

  `Vec<T>`の位置indexに要素を挿入し、それ以降のすべての要素を右にシフトします。

- Example

```rust
  let mut vec = vec![1, 2, 3];
  vec.insert(1, 4);
  assert_eq!(vec, [1, 4, 2, 3]);
  vec.insert(4, 5);
  assert_eq!(vec, [1, 4, 2, 3, 5]);
```


---

#### std::vec::Vec::reserve

- Description

  与えられた`Vec<T>`に挿入される少なくとも追加の要素のための容量を確保します。コレクションは、頻繁な再割り当てを避けるために、より多くの容量を確保することができます。`reserve`を呼び出した後の容量は、`self.len() + additional`以上になります。容量がすでに十分な場合は何もしません。

- Panics

  新しい容量が`isize::MAX`バイトを超えるとパニックになります。

- Example

```rust
  let mut vec = vec![1];
  vec.reserve(10);
  assert!(vec.capacity() >= 11);
```


---

#### std::vec::Vec::append

- Description

  他方の要素をすべて自分自身に移動させ、他方を空にします。

- Panics

  ベクタの要素数が`usize`を超えた場合、パニックを起こします。

- Example

```rust
  let mut vec = vec![1, 2, 3];
  let mut vec2 = vec![4, 5, 6];
  vec.append(&mut vec2);
  assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
  assert_eq!(vec2, []);
```
