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

---

### github

* [ghmagazine](https://github.com/ghmagazine/rustbook)
* [forcia](https://github.com/forcia/rustbook)
* [github.com/ytakasugi/](https://github.com/ytakasugi)

---

### 競技プログラミング

* [Atcoder Problems](https://kenkoooo.com/atcoder/#/table/)
* [過去問精選 10 問](https://qiita.com/drken/items/fd4e5e363d0f5859067)
* [RustCoder](https://zenn.dev/toga/books/rust-atcoder)
* [競プロ 典型90問](https://github.com/E869120/kyopro_educational_90)
* [競プロ典型90問をRustで解く](https://dev.thanaism.com/tags/rust/)

---

### rust doc
* [StandardLibrary](https://doc.rust-lang.org/std/)
* [async_std](https://docs.rs/async-std/1.9.0/async_std/)
* [Rust by Example](https://doc.rust-jp.rs/rust-by-example-ja/)
* [The Rust Programming Language](https://doc.rust-jp.rs/book-ja/)
* [rust-lang-ja](https://github.com/rust-lang-ja)
* [the-rust-programming-language-ja](https://github.com/rust-lang-ja/the-rust-programming-language-ja)
* [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)
* [Rust チートシート](https://cheats.rs/)
* [Plotters Developer Guide](https://plotters-rs.github.io/book/intro/introduction.html)
* [Tour of Rust](https://tourofrust.com/00_ja.html)
* [rustlings](https://github.com/rust-lang/rustlings)

---

### Applications

- [Tokio(JP)](https://zenn.dev/magurotuna/books/tokio-tutorial-ja)
- [Rustではじめるレイトレーシング入門](https://github.com/mebiusbox/docs/blob/master/Rust%E3%81%A7%E3%81%AF%E3%81%98%E3%82%81%E3%82%8B%E3%83%AC%E3%82%A4%E3%83%88%E3%83%AC%E3%83%BC%E3%82%B7%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80.pdf)
- [Rustでつくるインタプリタ](https://qiita.com/nirasan/items/f7a232af3372ea370f4b)
- [Command Line Toolを作ってみる in Rust](https://qiita.com/watawuwu/items/b20abfae62f76e4b4c0c)
- [Rustでheadコマンドを作ってみる](https://nktafuse.hatenablog.com/entry/2017/12/19/202823)
- [Rustで書くUDPサーバー](https://zenn.dev/psyashes/articles/794f73304b0350)
- [RustでWebアプリケーションを作る](https://caddi.tech/archives/416)

---

### Reference Site

* [Re:FizzBuzzから始めるRust生活](https://qiita.com/hinastory/items/543ae9749c8bccb9afbc)
* [Rust入門](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/6d5875)
* [とほほのWWW入門](http://www.tohoho-web.com/ex/rust.html)
* [`impl Trait`について](https://qnighy.hatenablog.com/entry/2018/01/28/220000)
* [本家Rustコンパイラのソースを読もうとしてみる（1）](https://qiita.com/0yoyoyo/items/eba97a019d0e60324263)
* [Rustのライフタイムについてのよくある誤解](https://github.com/pretzelhammer/rust-blog/blob/master/posts/translations/jp/common-rust-lifetime-misconceptions.md)
* [『Rust Design Patterns』を翻訳してみました（デザインパターン・アンチパターン編）](https://qiita.com/Yappii_111/items/654717e6a6a980722189)
* [Rust の最初のステップ](https://docs.microsoft.com/ja-jp/learn/paths/rust-first-steps/)
* [Rust を始めるための資料集](https://blog-dry.com/entry/2021/01/23/141936)
* [Rustのパターンマッチを完全に理解した](https://frozenlib.net/blog/2018-03-11_rust-pattern-match/)
* [Rust における`From<T>`とか`Into<T>`とかの考え方](https://qiita.com/hadashiA/items/d0c34a4ba74564337d2f)
* [Rustの`std::convert`の`From`とか`Into`トレイトがわからなくなった時に見る記事](https://qiita.com/SenK/items/b42b4dc95ab979098f12)
* [Rustのライフタイムを理解する](https://qiita.com/lechatthecat/items/863198824bbb8c4ab1f4)
* [Rustはこうやって勉強するといいんじゃないか、という一例](https://qiita.com/TakaakiFuruse/items/13e9ad9d1efe7e17811c)
* [構造体フィールドの所有権の部分借用/移動を理解する](https://qiita.com/yosqueoy/items/453e9aa85bf394388b86)
* [Rust のポリモルフィズムとトレイトオブジェクト](http://sharply.hatenablog.com/entry/2020/04/19/001236)
* [RustのSizedとfatポインタ](https://qnighy.hatenablog.com/entry/2017/03/04/131311)
* [Rustの便利クレート](https://qiita.com/qryxip/items/7c16ab9ef3072c1d7199)
* [Rustの型変換](https://zenn.dev/take4s5i/articles/rust-type-convertion)
* [Rustのトレイトを使おう!(1)](https://zenn.dev/naughie/articles/0da40abd7bf3cf)
* [Rust入門日記](https://zenn.dev/kawahara/scraps/5a22db01d86ec9)
* [Rustのイテレータの網羅的かつ大雑把な紹介](https://qiita.com/lo48576/items/34887794c146042aebf1)
* [RustのOwnershipってなんなん](https://zenn.dev/ucwork/articles/cfe579cbf5647e)
* [RustのLifetimeってなんなん](https://zenn.dev/ucwork/articles/6de5c9c2257f2d)
* [【初心者入門】Rustメモリ管理(所有権、借用、ライフタイム)](https://qiita.com/akito_tameto/items/a6840328224536e526a6)
* [Rust勉強会](https://scrapbox.io/nomlab/Rust%E5%8B%89%E5%BC%B7%E4%BC%9A)
* [Rustで新しくstructやenum を定義するときに実装を検討するtrait](https://qiita.com/magicant/items/1923d4f8f87a710d97b1)
* [Rust初心者が自動型変換や型変換関係のトレイトを自信を持って扱えるようになるための型変換まとめ8パターン](https://qiita.com/nirasan/items/e9c621240a7aae914cb8)
* [マルチスレッドプログラミングのメモ](https://totem3.hatenablog.jp/entry/2017/05/10/210000)
* [Rustはどのようにして安全な並列処理を提供するのか](https://qiita.com/nirasan/items/97263103f076bd525a7b)
* [Rustの非同期プログラミングをマスターする](https://tech-blog.optim.co.jp/entry/2019/11/08/163000)

---

### diesel

* [diesel公式サイト](https://diesel.rs/guides/getting-started/)

---

### other

- [cargo-clean-recursive](https://crates.io/crates/cargo-clean-recursive)
- [OPTiM TECH BLOG(Rust)](https://tech-blog.optim.co.jp/archive/category/Rust)
- [zenn(rustlang)](https://zenn.dev/topics/rust?order=latest)
- [Rust言語 2021年版の計画（抄訳）](https://zenn.dev/ice_creamer/articles/53c12111ab8d4b)
- [Rustup 1.24.2 について（抄訳）](https://zenn.dev/ice_creamer/articles/a57addc6d5e200)