### std::path

---

#### std::path::Path

  - Discription

    パスのスライス。

    この型はパスを検査するための多くの操作をサポートしている。パスをその構成要素に分割したり(Unixでは/で区切って、Windowsでは/または/で区切って)、ファイル名を抽出したり、パスが絶対パスかどうかを判断したりなど。

    非サイズ型であり、常に 参照や`Box`のようなポインタの後ろで使用されなければならない。

---

#### std::path::Path::new

- Description

  文字列スライスを`Path`スライスとして直接ラップします。

  これはコストのかからない変換です。

---

#### std::path::PathBuf

- Description

  所有権のある、変更可能なパス（Stringに似ている）。

  このタイプには、パスをその場で変更する`push`や`set_extension`などのメソッドがあります。また、`Path`への`Deref`を実装しており、`Path`スライス上のすべてのメソッドは、`PathBuf`の値でも利用可能であることを意味します。

  全体的なアプローチについての詳細は、モジュールのドキュメントに記載されています。

- Example

  `push`を使ってコンポーネントから`PathBuf`を構築することができます。

```rust
  use std::path::PathBuf;
  
  let mut path = PathBuf::new();
  
  path.push(r"C:\");
  path.push("windows");
  path.push("system32");
  
  path.set_extension("dll");
```

  しかし、`push`は動的な状況で使用するのがベストです。これは、すべてのコンポーネントを事前に知っている場合には、より良い方法です。

```rust
  use std::path::PathBuf;
  
  let path: PathBuf = [r"C:\", "windows", "system32.dll"].iter().collect();
```

  これよりももっと良い方法があります。これらはすべて文字列なので、`From::from`を使うことができます。

```rust
  use std::path::PathBuf;
  
  let path = PathBuf::from(r"C:\windows\system32.dll");
```

  どの方法が一番効果的かは、どのような状況にあるかによって異なります。

