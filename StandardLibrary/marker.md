i### std::marker

---

#### std::marker::Sync

- Description

  スレッド間で参照を共有することが安全な型。

  この特性は、コンパイラが適切と判断した場合に自動的に実装されます。

  正確な定義は次のとおりです：型Tは、`&T`が`Send`である場合に限り、`Sync`です。言い換えれば、スレッド間で`&T`の参照を渡す際に未定義の動作（データレースを含む）が発生する可能性がない場合です。

  予想されるように、`u8`や`f64`のようなプリミティブな型はすべて`Sync`であり、タプルや構造体、`enum`のようなそれらを含む単純な集約型も同様です。基本的な Sync 型の例としては、`&T`のような「不変」型や、`Box<T>`, `Vec<T>`や他のほとんどのコレクション型のような単純な継承された可変型があります。(ジェネリックパラメータは、そのコンテナが`Sync`であるために`Sync`である必要があります。)

  この定義のやや意外な結果は、非同期のミューテーションを提供しているように見えても、`&mut T`は (`T`が`Sync`であれば) `Sync`であるということです。このトリックは、共有参照の背後にある変異可能な参照（つまり`& &mut T`）が、あたかも`& &T `のように読み取り専用になることです。したがって、データレースのリスクはありません。

  `Sync`でない型は、`Cell`や`RefCell`のように、スレッドセーフでない形で「内部の変異性」を持っているものです。これらの型は、不変の共有参照を通しても、その内容の変異を許します。例えば、`Cell<T>`の`set`メソッドは、`&self`を受け取るので、共有参照の`&Cell<T>`だけが必要です。このメソッドは同期を行わないので、`Cell`は`Sync`にはなりません。

  非`Sync`型のもうひとつの例は、参照カウントポインタ`Rc`です。任意の参照 `&Rc<T> `が与えられると、新しい`Rc<T>`をクローンし、非アトミックな方法で参照カウントを変更することができます。

  スレッドセーフな内部の変異性が必要な場合、Rustはアトミックなデータ型を提供し、`sync::Mutex`と`sync::RwLock`による明示的なロックも提供します。`sync::Mutex`と`sync::RwLock`による明示的なロックがあります。これらのデータ型は、いかなる変異もデータレースを引き起こさないことを保証します。同様に、`sync::Arc`は`Rc`のスレッドセーフな類似性を提供します。

  内部に変異性を持つ型は、共有参照を通して変異させることができる値の周りに `cell::UnsafeCell`ラッパーを使用しなければなりません。これを怠ると未定義の動作となります。例えば、`&T`から`&mut T`への変換は無効です。

  `Sync`の詳細については[Nomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html)を参照してください。

---

#### std::marker::Send

- Description

  スレッドの境界を越えて転送可能な型。

  この特性は、コンパイラが適切と判断した場合、自動的に実装されます。

  非`Send`型の例としては、参照カウントのポインタ`rc::Rc`があります。2つのスレッドが同じ参照カウント値を指す`Rc`を複製しようとすると、同時に参照カウントを更新しようとするかもしれませんが、`Rc`はアトミック演算を使用していないので、これは未定義の動作です。これは`Rc`がアトミックな操作をしないため、未定義の動作です。

  詳しくは[Nomicon](https://doc.rust-lang.org/nomicon/send-and-sync.html)をご覧ください。

---

#### std::marker::PhantomData

- Description

  `T`型を所有しているかのように「振る舞う」ものをマークするために使用されるゼロサイズの型。
  型に`PhantomData<T>`フィールドを追加すると、実際には`T`型の値を格納していないにもかかわらず、あたかも`T`型の値を格納しているかのように振る舞うことをコンパイラに伝えます。この情報は、特定の安全プロパティを計算する際に使用されます。
  PhantomData<T>の使用方法については、[Nomicon](https://doc.rust-lang.org/nomicon/phantom-data.html)を参照してください。

- Example

  - Unused lifetime parameters

    おそらくPhantomDataの最も一般的な使用例は、未使用の寿命パラメータを持つ構造体で、通常は安全でないコードの一部として使用されます。例えば、ここには`*const T`型の2つのポインタを持つ`Slice`構造体があり、おそらくどこかの配列を指していると思われます。

~~~rust
    struct Slice<'a, T> {
        start: *const T,
        end: *const T,
    }
~~~

    この意図は、基礎となるデータはライフタイム`'a`に対してのみ有効なので、`Slice`は`'a`よりも長生きしてはいけないということです。しかし、この意図はコードでは表現されていません。ライフタイム`'a`の用途がないため、どのデータに適用されるのかが明確ではありません。これを修正するには、コンパイラに`Slice`構造体に参照`&'a T`が含まれているかのように動作するように指示します。

~~~rust
    use std::marker::PhantomData;
    
    struct Slice<'a, T: 'a> {
        start: *const T,
        end: *const T,
        phantom: PhantomData<&'a T>,
    }
~~~

    これにより、`T: 'a`というアノテーションが必要になり、T内の参照が有効期間`'a`にわたって有効であることを示します。
    `Slice`を初期化する際には、`Phantom`フィールドに`PhantomData`という値を指定するだけです。

~~~rust
    fn borrow_vec<T>(vec: &Vec<T>) -> Slice<'_, T> {
        let ptr = vec.as_ptr();
        Slice {
            start: ptr,
            end: unsafe { ptr.add(vec.len()) },
            phantom: PhantomData,
        }
    }
~~~

  - Unused type parameters

    構造体自体にはデータが存在しないにもかかわらず、未使用の型パラメータが存在し、構造体がどのようなデータに「関連付けられているか」を示すことがあります。ここでは、`FFI`でこのような問題が発生する例を示します。外部インターフェイスでは、異なるタイプの`Rust`値を参照するために`*mut()`型のハンドルを使用します。ハンドルをラップする`ExternalResource`構造体のファントム型パラメータを使用して`Rust`型を追跡します。

~~~rust
    use std::marker::PhantomData;
    use std::mem;
    
    struct ExternalResource<R> {
       resource_handle: *mut (),
       resource_type: PhantomData<R>,
    }
    
    impl<R: ResType> ExternalResource<R> {
        fn new() -> Self {
            let size_of_res = mem::size_of::<R>();
            Self {
                resource_handle: foreign_lib::new(size_of_res),
                resource_type: PhantomData,
            }
        }
    
        fn do_stuff(&self, param: ParamType) {
            let foreign_params = convert_params(param);
            foreign_lib::do_stuff(self.resource_handle, foreign_params);
        }
    }
~~~

  - Ownership and the drop check

    `PhantomData<T>`型のフィールドを追加することは、あなたの型が`T`型のデータを所有していることを示します。これは、あなたの型がドロップされたときに、`T`型の1つ以上のインスタンスをドロップする可能性があることを意味しています。これは、`Rust`コンパイラのドロップチェック解析に関係します。
    構造体が実際に`T`型のデータを所有していない場合は、所有権を示さないように`PhantomData<&'a T>`(理想的には)または`PhantomData<*const T>`(ライフタイムが適用されない場合)のような参照型を使用した方が良いでしょう。

