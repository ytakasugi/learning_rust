### 関数ポインタ

関数を指す（ポイントする）変数束縛も作れます。

```rust
let f: fn(i32) -> i32;
```

`f`という変数束縛は、`i32`を引数として受け取り、`i32`を返す関数へのポインタになります。 例えばこうです。

```rust
fn plus_one(i: i32) -> i32 {
    i + 1
}

// 型推論なし
let f: fn(i32) -> i32 = plus_one;

// 型推論あり
let f = plus_one;
```

そして`f`を使って関数を呼び出せます。

```rust
fn plus_one(i: i32) -> i32 { i + 1 }
let f = plus_one;
let six = f(5);
```

---

### エラーハンドリング

- `Option`型

`Option` 型は、Rustの型システムを使って *不在の可能性* を示すためのものです。 不在の可能性を型システムにエンコードすることは、重要なコンセプトです。 なぜなら、その不在に対処することを、コンパイラがプログラマに強制させるからです。 では、文字列から文字を検索する例を見てみましょう。

```rust
// 文字列から文字を検索する関数
// 文字列スライスとcharを引数にとり、`Option`型を返す
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}

fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        // 第二引数以降の文字列を抽出
        Some(i) => println!("File extension: {}", &file_name[i + 1..]),
    }
}
```

- `Option<T>`値を合成する

ファイル名から拡張子を見つけるために `find` をどのように使うかを見ました。 当然ながら全てのファイル名に `.` があるわけではなく、拡張子のないファイル名もあり得ます。 このような *不在の可能性* は `Option<T>` を使うことによって、型の中にエンコードされています。 すなわち、コンパイラは、拡張子が存在しない可能性に対処することを、私たちに強制してくるわけです。 今回は単に、そうなったことを告げるメッセージを表示するようにしました。

ファイル名から拡張子を取り出すことは一般的な操作ですので、それを関数にすることは理にかなっています。

```rust
fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
    }
}
```

注目すべきは、`find` の型が不在の可能性について考慮することを強制していることです。 これは良いことです。なぜなら、コンパイラが私たちに、ファイル名が拡張子を持たないケースを、うっかり忘れないようにしてくれるからです。 しかし一方で、 `extension_explicit` でしたような明示的な場合分けを毎回続けるのは、なかなか面倒です。

 `extension_explicit` での場合分けは、ごく一般的なパターンである、`Option<T>` への *map* の適用に当てはめられます。 これは、もしオプションが `None` なら `None` を返し、そうでなけれは、オプションの中の値に関数を適用する、というパターンです。

Rustはパラメトリック多相をサポートしていますので、このパターンを抽象化するためのコンビネータが簡単に定義できます

```rust
fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}
```

 `map` は、標準のライブラリの `Option<T>` で [メソッドとして定義されています](https://github.com/rust-lang-ja/the-rust-programming-language-ja/blob/master/1.9/ja/std/option/enum.Option.html#method.map)。 メソッドなので、少し違うシグネチャを持っています。 メソッドは第一引数に `self` 、 `&self` あるいは `&mut self` を取ります。
 
`extension_explicit` メソッドを書き直して、場合分けを省きましょう

```rust
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}
```



---

- Others

GoをWSL2にインストールする手順を残しておく(Rustは関係ない)

```
$ wget https://golang.org/dl/go1.16.6.linux-amd64.tar.gz

$ vi ~/.bashrc
# add lines
export GOROOT=/usr/local/go
export GOPATH=$HOME/go
export PATH=$GOPATH/bin:$GOROOT/bin:$PATH

$ source ~/.bashrc

$ go version
```
