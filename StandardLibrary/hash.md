### std::hash

---

#### `std::hash::Hash`

- Description

  ハッシュ化可能な型です。

  Hashを実装した型は、Hasherのインスタンスでハッシュ化することができます。

- Implementing `Hash`

  すべてのフィールドがHashを実装していれば、`#[derive(Hash)]`で`Hash`を派生させることができます。結果として得られるハッシュは、各フィールドのハッシュを呼び出したときの値を組み合わせたものになります。

~~~rust
  #[derive(Hash)]
  struct Rustacean {
      name: String,
      country: String,
  }
~~~

  値がどのようにハッシュ化されるかをより細かく制御する必要がある場合は、もちろん自分でHash特性を実装することができます。

~~~rust
  use std::hash::{Hash, Hasher};
  
  struct Person {
      id: u32,
      name: String,
      phone: u64,
  }
  
  impl Hash for Person {
      fn hash<H: Hasher>(&self, state: &mut H) {
          self.id.hash(state);
          self.phone.hash(state);
      }
  }
~~~

- `Hash` and `Eq`

  HashとEqの両方を実装する際には、次のような性質が成り立つことが重要です。

~~~
  k1 == k2 -> hash(k1) == hash(k2)
~~~

  言い換えれば、2つのキーが等しい場合、それらのハッシュも等しくなければなりません。HashMapとHashSetはどちらもこの動作に依存しています。

  ありがたいことに、`#[derive(PartialEq, Eq, Hash)]`で`Eq`と`Hash`の両方を導出する際には、このプロパティの保持を心配する必要はありません。

- Required methods

~~~rust
  pub fn hash<H>(&self, state: &mut H)
  where
      H: Hasher,
~~~

  このタイプのスライスを与えられたHasherにフィードします。

  - Example

    ---

~~~rust
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    7920.hash(&mut hasher);
    println!("Hash is {:x}!", hasher.finish());
~~~

- Provided methods

~~~rust
  pub fn hash_slice<H>(data: &[Self], state: &mut H)
  where
      H: Hasher, 
  
~~~

  このタイプのスライスを与えられたHasherにフィードします。

  - Exmaple

    ---

~~~rust
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    let numbers = [6, 28, 496, 8128];
    Hash::hash_slice(&numbers, &mut hasher);
    println!("Hash is {:x}!", hasher.finish());
~~~



---

#### `std::hash::Hasher`

- Description

  任意のバイトストリームをハッシュ化するための`trait`です。

  `Hasher`のインスタンスは通常、データのハッシュ化の際に変更される状態を表します。

  `Hasher`は、生成されたハッシュを (`finish`を使って) 取得したり、整数やバイトのスライスを (`write`や`write_u8`などを使って) インスタンスに書き込むための、かなり基本的なインターフェースを提供します。ほとんどの場合、`Hasher`インスタンスは、`Hash trait`と一緒に使用されます。

- Example

~~~rust
  use std::collections::hash_map::DefaultHasher;
  use std::hash::Hasher;
  
  let mut hasher = DefaultHasher::new();
  
  hasher.write_u32(1989);
  hasher.write_u8(11);
  hasher.write_u8(9);
  hasher.write(b"Huh?");
  
  println!("Hash is {:x}!", hasher.finish());
~~~
