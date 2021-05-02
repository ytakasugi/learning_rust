* New-Item -type file [ファイル名]

* sudo apt install pkg-config

* sudo apt-get install libssl-dev

* Atcoder

  * Cargo.toml

  ```
  [dependencies]
  proconio = "0.4.1"
  ```

  * main.rs

  ```rust
  use proconio::input;
  ```

  * 入出力マクロ

  ```rust
  use std::io::*;
  use std::str::FromStr;

  fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char) 
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
  }
  ```

### github

* [ghmagazine](https://github.com/ghmagazine/rustbook)

* [forcia](https://github.com/forcia/rustbook)

* [ytakasugi/workspace](https://github.com/ytakasugi/workspace)

### Atcoder

* [Atcoder Problems](https://kenkoooo.com/atcoder/#/table/)

* [過去問精選 10 問](https://qiita.com/drken/items/fd4e5e363d0f5859067)

### rust doc
* [StandardLibrary](https://doc.rust-lang.org/stable/std/)

### Reference site

* [Re:FizzBuzzから始めるRust生活](https://qiita.com/hinastory/items/543ae9749c8bccb9afbc)

* [Rust チートシート](https://cheats.rs/)

* [Tokio(JP)](https://zenn.dev/magurotuna/books/tokio-tutorial-ja)

* [Rust入門](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/6d5875)

* [Rustではじめるレイトレーシング入門](https://github.com/mebiusbox/docs/blob/master/Rust%E3%81%A7%E3%81%AF%E3%81%98%E3%82%81%E3%82%8B%E3%83%AC%E3%82%A4%E3%83%88%E3%83%AC%E3%83%BC%E3%82%B7%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80.pdf)

* [async_std](https://docs.rs/async-std/1.9.0/async_std/)

* [とほほのWWW入門](http://www.tohoho-web.com/ex/rust.html)

* [`impl Trait`について](https://qnighy.hatenablog.com/entry/2018/01/28/220000)

* [本家Rustコンパイラのソースを読もうとしてみる（1）](https://qiita.com/0yoyoyo/items/eba97a019d0e60324263)

* [Rustのライフタイムについてのよくある誤解](https://github.com/pretzelhammer/rust-blog/blob/master/posts/translations/jp/common-rust-lifetime-misconceptions.md)

* [RustCoder](https://zenn.dev/toga/books/rust-atcoder)

* [競プロ 典型90問](https://github.com/E869120/kyopro_educational_90)

* [『Rust Design Patterns』を翻訳してみました（デザインパターン・アンチパターン編）](https://qiita.com/Yappii_111/items/654717e6a6a980722189)

* [Rust の最初のステップ](https://docs.microsoft.com/ja-jp/learn/paths/rust-first-steps/)

* [Rust を始めるための資料集](https://blog-dry.com/entry/2021/01/23/141936)

* [Rustのパターンマッチを完全に理解した](https://frozenlib.net/blog/2018-03-11_rust-pattern-match/)

* [Plotters Developer Guide](https://plotters-rs.github.io/book/intro/introduction.html)

* [競プロ典型90問をRustで解く](https://dev.thanaism.com/tags/rust/)

### diesel

* [diesel公式サイト](https://diesel.rs/guides/getting-started/)
