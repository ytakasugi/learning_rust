### std::any

- Description

  このモジュールは、`Any trait`を実装しています。`Any trait`は、ランタイムリフレクションによって任意の「静的な型」の動的な型付けを可能にします。

  `Any`自身は`TypeId`を取得するために使用できますが、`trait`オブジェクトとして使用するとさらに多くの機能があります。`&dyn Any`(借用した trait オブジェクト) としては、`is`および `downcast_ref `メソッドがあり、含まれる値が所定の型であるかどうかをテストしたり、内部の値を型として参照することができます。また、`&mut dyn Any`として、内部値への`mutable`な参照を取得する`downcast_mut`メソッドもあります。`Box<dyn Any>`には、`Box<T>`への変換を試みる`downcast`メソッドが追加されています。詳細は`Box`のドキュメントを参照してください。

  なお、`&dyn Any`は、値が指定された具象型であるかどうかのテストに限定されており、型が`trait`を実装しているかどうかのテストには使用できません。

- Example

  関数に渡された値をログアウトさせたい場合を考えてみましょう。対象となる値がDebugを実装していることはわかっていますが、その具体的な型はわかりません。特定の型に対して特別な扱いをしたいと考えています。この例では、`String`値の長さを値の前に表示しています。コンパイル時には値の具体的な型がわからないので、代わりにランタイム・リフレクションを使用する必要があります。

~~~rust
  use std::fmt::Debug;
  use std::any::Any;
  
  // Logger function for any type that implements Debug.
  fn log<T: Any + Debug>(value: &T) {
      let value_any = value as &dyn Any;
  
      // Try to convert our value to a `String`. If successful, we want to
      // output the String`'s length as well as its value. If not, it's a
      // different type: just print it out unadorned.
      match value_any.downcast_ref::<String>() {
          Some(as_string) => {
              println!("String ({}): {}", as_string.len(), as_string);
          }
          None => {
              println!("{:?}", value);
          }
      }
  }
  
  // This function wants to log its parameter out prior to doing work with it.
  fn do_work<T: Any + Debug>(value: &T) {
      log(value);
      // ...do some other work
  }
  
  fn main() {
      let my_string = "Hello World".to_string();
      do_work(&my_string);
  
      let my_i8: i8 = 100;
      do_work(&my_i8);
  }
~~~

---

#### std::any::TypeId

- Description

  `TypeId`は、あるタイプのグローバルに一意な識別子を表します。

  各`TypeId`は不透明なオブジェクトで、中身を確認することはできませんが、複製、比較、印刷、表示などの基本的な操作は可能です。

  `TypeId`は現在、'static'に準拠した型でのみ利用可能ですが、この制限は将来的に削除される可能性があります。

  `TypeId`は`Hash`、`PartialOrd`、`Ord`を実装していますが、ハッシュや順序は`Rust`のリリースごとに異なることに注意してください。コードの中でこれらに依存することに注意してください。

- Implementation

~~~rust
  pub fn of<T>() -> TypeId
  where
      T: 'static + ?Sized, 
  
~~~

  このジェネリック関数がインスタンス化された型のTypeIdを返します。

  - Example

    ---

~~~rust
    use std::any::{Any, TypeId};
    
    fn is_string<T: ?Sized + Any>(_s: &T) -> bool {
        TypeId::of::<String>() == TypeId::of::<T>()
    }
    
    assert_eq!(is_string(&0), false);
    assert_eq!(is_string(&"cookie monster".to_string()), true);
~~~
