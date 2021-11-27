### slice

---

#### slice::windows

- Description

  長さ`size`のすべての連続したウィンドウのイテレータを返します。ウィンドウは重なっています。スライスがsize`よりも短い場合、イテレータは値を返しません。

- Example

```rust
  let slice = ['r', 'u', 's', 't'];
  let mut iter = slice.windows(2);
  assert_eq!(iter.next().unwrap(), &['r', 'u']);
  assert_eq!(iter.next().unwrap(), &['u', 's']);
  assert_eq!(iter.next().unwrap(), &['s', 't']);
  assert!(iter.next().is_none());
```

  

---

#### slice::contains

- Descriptiom

  スライスに指定した値の要素が含まれている場合はtrueを返します。

---

#### slice::starts_with

- Description

  needleがスライスの接頭辞である場合にtrueを返します。

---

#### slice::ends_with

- Description

  needleがスライスの接尾辞である場合にtrueを返します。



---

#### slice::sort

- Description

  スライスをソートします。

  このソートは安定しており（すなわち、等しい要素を並べ替えない）、最悪の場合`O(n * log(n))`となります。

  不安定なソートは、一般的に安定したソートよりも高速で、補助的なメモリを割り当てないので、適用可能な場合は、不安定なソートが好ましいです。[`sort_unstable`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort_unstable)を参照してください。

- 現在の実装

  現在のアルゴリズムは，[`timsort`](https://en.wikipedia.org/wiki/Timsort)にヒントを得た適応的な反復型マージソートです．これは、スライスがほぼソートされている場合や、2つ以上のソートされたシーケンスを次々と連結して構成されている場合に、非常に高速になるように設計されています。

  また、`self`の半分の大きさの一時ストレージを割り当てますが、短いスライスの場合は、代わりに非割り当ての挿入ソートを使用します。

- Example

```rust
  let mut v = [-5, 4, 1, -3, 2];
  
  v.sort();
  assert!(v == [-5, -3, 1, 2, 4]);
```

  

---

#### slice::sort_by

- Description

  スライスをコンパレータ関数でソートします。

  このソートは安定しており（つまり、等しい要素を並べ替えない）、最悪の場合は`O(n * log(n))`です。

  コンパレータ関数は、スライス内の要素の合計順序を定義しなければなりません。順序が合計でない場合、要素の順序は指定されません。順番は、（すべての`a`、`b`、`c`について）である場合、合計の順番です。

  - total and antisymmetric: `a < b`, `a == b`, `a > b` のうちの1つが正確に真で
  - transitive:`a < b` と`b < c` が含まれるとき `a < c`. 同じことが`==`と`>`にも当てはまります。

  例えば、`f64`は`NaN != NaN`なので`Ord`を実装していませんが、スライスに`NaN`が含まれていないことがわかっている場合は、`partial_cmp`をソート関数として使用することができます。

```rust
  let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
  floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
  assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);
```

  不安定なソートは、一般的に安定したソートよりも高速で、補助的なメモリを割り当てないため、適用可能な場合は、不安定なソートが推奨されます。[`sort_unstable_by`](https://doc.rust-lang.org/stable/std/primitive.slice.html#method.sort_unstable_by) を参照してください。
  
- 現在の実装

  現在のアルゴリズムは，[`timsort`](https://en.wikipedia.org/wiki/Timsort)にヒントを得た適応的な反復型マージソートです．これは、スライスがほぼソートされている場合や、2つ以上のソートされたシーケンスを次々と連結して構成されている場合に、非常に高速になるように設計されています。

  また、`self`の半分の大きさの一時ストレージを割り当てますが、短いスライスの場合は、代わりに非割り当ての挿入ソートを使用します。

- Example

```rust
  let mut v = [5, 4, 1, 3, 2];
  v.sort_by(|a, b| a.cmp(b));
  assert!(v == [1, 2, 3, 4, 5]);
  
  // reverse sorting
  v.sort_by(|a, b| b.cmp(a));
  assert!(v == [5, 4, 3, 2, 1]);
```

---

#### slice::sort_by_key

- Description

  スライスをキー抽出関数でソートします。

  このソートは安定しており（すなわち、等しい要素を並べ替えない）、キー関数が`O(m)`である場合、最悪のケースでは`O(m * n * log(n))`となります。

  高価なキー関数（単純なプロパティアクセスや基本操作ではない関数など）では、要素のキーを再計算しないため、`sort_by_cached_key`の方が大幅に高速になる可能性があります。

  不安定なソートは、一般的に安定したソートよりも高速で、補助メモリを割り当てないため、適用可能な場合は、不安定なソートが好ましい。`sort_unstable_by_key`を参照してください。

- Current implementation

  現在のアルゴリズムは，[`timsort`](https://en.wikipedia.org/wiki/Timsort)にヒントを得た適応的な反復型マージソートです．これは、スライスがほぼソートされている場合や、2つ以上のソートされたシーケンスを次々と連結して構成されている場合に、非常に高速になるように設計されています。

  また、`self`の半分のサイズの一時ストレージを割り当てますが、短いスライスの場合は、代わりに非割り当ての挿入ソートを使用します。

- Example

```rust
  let mut v = [-5i32, 4, 1, -3, 2];
  
  v.sort_by_key(|k| k.abs());
  assert!(v == [1, 2, -3, 4, -5]);
```



---

#### slice::iter_mut

- Description

  各値を変更できるようにするイテレータを返します。



---

#### slice::iter

- Description

  スライスのイテレータを返します。

- Example

```rust
  let x = &[1, 2, 4];
  let mut iterator = x.iter();
  
  assert_eq!(iterator.next(), Some(&1));
  assert_eq!(iterator.next(), Some(&2));
  assert_eq!(iterator.next(), Some(&4));
  assert_eq!(iterator.next(), None);
```




---

#### slice::get

- Descroption

  インデックスのタイプに応じて、要素またはサブスライスへの参照を返します。

  - 位置が指定されている場合は、その位置にある要素への参照を返します。
  - 範囲が指定されている場合は、その範囲に対応するサブスライスを返し、範囲外の場合は`None`を返します。

- Example

```rust
  let v = [10, 40, 30];
  assert_eq!(Some(&40), v.get(1));
  assert_eq!(Some(&[10, 40][..]), v.get(0..2));
  assert_eq!(None, v.get(3));
  assert_eq!(None, v.get(0..4));
```



---

#### slice::join

- Description

  `T`のスライスを単一の値`Self::Output`にフラット化し、それぞれの間に指定されたセパレータを配置します。

- Example

```rust
  assert_eq!(["hello", "world"].join(" "), "hello world");
  assert_eq!([[1, 2], [3, 4]].join(&0), [1, 2, 0, 3, 4]);
  assert_eq!([[1, 2], [3, 4]].join(&[0, 0][..]), [1, 2, 0, 0, 3, 4]);
```




---

#### slice::into_vec

- Description

  クローンやアロケーションを使わずに`self`を`vector`に変換します。

  変換されたベクトルは、`Vec<T>`の`into_boxed_slice`メソッドによって、再び`Box<T>`に変換することができます。

- Example

```rust
  let s: Box<[i32]> = Box::new([10, 40, 30]);
  let x = s.into_vec();
  // `s` cannot be used anymore because it has been converted into `x`.
  
  assert_eq!(x, vec![10, 40, 30]);
```
