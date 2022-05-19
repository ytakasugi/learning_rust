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
* [rustviz](https://github.com/rustviz/rustviz)
* [rust blog](https://github.com/pretzelhammer/rust-blog)

---

### プログラムがメモリをどう使うかを理解する
      
* [その1](https://zenn.dev/rita0222/articles/e6ff75245d79b5)
* [その2](https://zenn.dev/rita0222/articles/beda4311d9a6bf)
* [その3](https://zenn.dev/rita0222/articles/f59b79bab45a2a)
* [その4](https://zenn.dev/rita0222/articles/1f37a5bf910282)

---

### CPU

* [CPU使用率とは何か](https://t-keita.hatenadiary.jp/entry/2021/11/18/014135)

---

### WEB

* [Webデザイン初心者向け Webサイトの作り方と準備](https://web-design-textbook.com/)

---

### 競技プログラミング

* [Atcoder Problems](https://kenkoooo.com/atcoder/#/table/)
* [過去問精選 10 問](https://qiita.com/drken/items/fd4e5e363d0f5859067)
* [RustCoder](https://zenn.dev/toga/books/rust-atcoder)
* [競プロ 典型90問](https://github.com/E869120/kyopro_educational_90)
* [競プロ典型90問をRustで解く](https://dev.thanaism.com/tags/rust/)
* [競技プログラミングでの典型アルゴリズムとデータ構造](https://algo-logic.info/competitive-programming-must/)
* [AtCoder Beginner Contestで最低限理解する必要がある（と感じた）数学的知識](https://qiita.com/Ll_e_ki/items/fe70b9e3408c5b14ae2e?utm_campaign=post_article&utm_medium=twitter&utm_source=twitter_share)
* [Rustで競技プログラミングを始めた人のためのデータ構造紹介](https://zenn.dev/tai_calg/articles/ecbd269503dd61)
* Rustで蟻本
    * [その1](https://qiita.com/mochafe/items/1076aa4effd148d7626f)
    * [その2](https://qiita.com/mochafe/items/daf30e19f85d3a42651e)
    * [Rustで蟻本　〜データ構造〜](https://qiita.com/mochafe/items/0bc5e26ab75fef875fe6)
* [Rustでも格子探索を楽したい！](https://qiita.com/jjitch/items/1e6f9861e506acf4abae) 

---

### rust doc
* [StandardLibrary](https://doc.rust-lang.org/std/)
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/about.html)
    * [非公式日本語訳](https://sinkuu.github.io/api-guidelines/about.html)
* [Edition Guide](https://doc.rust-jp.rs/edition-guide/)
* [async_std](https://docs.rs/async-std/1.9.0/async_std/)
* [Rust by Example](https://doc.rust-jp.rs/rust-by-example-ja/)
* [The Rust Programming Language](https://doc.rust-jp.rs/book-ja/)
  * [The Rust Programming Languageの問題を解いてみた　その１](https://qiita.com/kaclar_ef/items/7ccf3859f44554b5f0ac)
* [Guide to Rustc Development](https://rustc-dev-guide.rust-lang.org/)
* [Easy Rust](https://dhghomon.github.io/easy_rust/Chapter_0.html)
* [High Assurance Rust Developing Secure and Robust Software](https://highassurance.rs)
* [rust-lang-ja](https://github.com/rust-lang-ja)
* [The Rust Programming Language(旧和訳リポジトリ)](https://github.com/rust-lang-ja/the-rust-programming-language-ja/tree/master/1.9/ja/book)
* [the-rust-programming-language-ja](https://github.com/rust-lang-ja/the-rust-programming-language-ja)
* [The Rustonomicon](https://doc.rust-lang.org/nomicon/intro.html)
* [The Rust Reference](https://doc.rust-lang.org/reference/introduction.html)
* [This Week in Rust](https://this-week-in-rust.org/)
* [RustBelt](https://plv.mpi-sws.org/rustbelt/#publications)
* [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)
* [Rust チートシート](https://cheats.rs/)
* [Plotters Developer Guide](https://plotters-rs.github.io/book/intro/introduction.html)
* [Tour of Rust](https://tourofrust.com/00_ja.html)
* [rustlings](https://github.com/rust-lang/rustlings)
  * [解答例1](https://github.com/tenajima/tena_rustlings/tree/master/exercises)
  * [解答例2](https://github.com/takuchalle/rustlings-answer)
* [Crate csv](https://docs.rs/csv/1.1.6/csv/)
* [Overview Serde](https://serde.rs/)
* [Rust Design Patterns](https://rust-unofficial.github.io/patterns/intro.html)
  * [『Rust Design Patterns』を翻訳してみました（Idiom 編）](https://qiita.com/Yappii_111/items/4ccc3a8461cdd4035651)
  * [『Rust Design Patterns』を翻訳してみました（デザインパターン・アンチパターン編）](https://qiita.com/Yappii_111/items/654717e6a6a980722189)
* [Futures Explained in 200 Lines of Rust](https://cfsamson.github.io/books-futures-explained/introduction.html)
* [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/#working-with-multiple-error-types)
* [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/index.html)

---

### Applications

- [Tokio(JP)](https://zenn.dev/magurotuna/books/tokio-tutorial-ja)
- [Rustではじめるレイトレーシング入門](https://github.com/mebiusbox/docs/blob/master/Rust%E3%81%A7%E3%81%AF%E3%81%98%E3%82%81%E3%82%8B%E3%83%AC%E3%82%A4%E3%83%88%E3%83%AC%E3%83%BC%E3%82%B7%E3%83%B3%E3%82%B0%E5%85%A5%E9%96%80.pdf)
- [Rustでつくるインタプリタ](https://qiita.com/nirasan/items/f7a232af3372ea370f4b)
- [Command Line Toolを作ってみる in Rust](https://qiita.com/watawuwu/items/b20abfae62f76e4b4c0c)
- [Rustでheadコマンドを作ってみる](https://nktafuse.hatenablog.com/entry/2017/12/19/202823)
- [Rustで書くUDPサーバー](https://zenn.dev/psyashes/articles/794f73304b0350)
- [【翻訳】RustとCSV解析](https://qiita.com/algebroid/items/c456d4ec555ae04c7f92)
  - [サンプルデータ](https://github.com/BurntSushi/rust-csv/blob/master/examples/data/uspop.csv)
- [A TCP Proxy in 30 lines of Rust](https://zmedley.com/tcp-proxy.html)
- [ちいさなWebブラウザを作ろう](https://browserbook.shift-js.info/)
- [RESTful API in Sync & Async Rust](https://github.com/pretzelhammer/rust-blog/blob/master/posts/restful-api-in-sync-and-async-rust.md)
- [A Simple Web Server](https://ibraheem.ca/writings/a-simple-web-server/)
- [Getting Started with Systems Programming with Rust (Part 1)](https://dev.to/bexxmodd/getting-started-with-systems-programming-with-rust-part-1-2i13)
- [Practical Rust Web Development - Front-End](https://dev.to/werner/practical-rust-web-development-front-end-538d)
- [Rustで土日祝を色付きで出力するCLIツールを作ってみた](https://zenn.dev/mayo_dev/articles/rust-get-holiday-cli)
- [Making My Website Part 1: Monitoring On A Raspberry Pi](https://www.rotoclone.zone/blog/posts/raspberry-pi-monitoring)
- [Building My First Command Line Interface (CLI) with Rust](https://devtails.medium.com/building-my-first-command-line-interface-cli-with-rust-b6beb9c284e0)
- [Rustでsupabaseにサインアップする](https://zenn.dev/brainvader/articles/eef5630fcd88fb)
- [Rustで実装するmalloc](https://engineers.ntt.com/entry/2021/12/21/125459)
- [Rustで始める自作シェル その1](https://tech.buty4649.net/entry/2021/12/19/235124)
- Rst でパーサコンビネータを作ってみる
    - [前編](https://zenn.dev/nojima/articles/05cb9ffa0f993b)
    - [後編](https://zenn.dev/nojima/articles/e597d22660205d)

---

### Reference Site

* 入門記事
  * [Rust を始めるための資料集](https://blog-dry.com/entry/2021/01/23/141936)
  * [Rustを始める時に役立つ資料](https://qiita.com/kxkx5150/items/ff70c564c5c136ba3d25)
  * [35 Rust Learning Resources Every Beginner Should Know in 2022](https://apollolabsblog.hashnode.dev/35-rust-learning-resources-every-beginner-should-know-in-2022)
  * [Rust の最初のステップ](https://docs.microsoft.com/ja-jp/learn/paths/rust-first-steps/)
  * [Rust入門](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/6d5875)
  * [とほほのWWW入門](http://www.tohoho-web.com/ex/rust.html)
  * [Rust from the beginning, your first program](https://dev.to/azure/rust-from-the-beginning-your-first-program-30cp)
  * [You Want to Learn Rust but You Don’t Know Where to Start](https://towardsdatascience.com/you-want-to-learn-rust-but-you-dont-know-where-to-start-fc826402d5ba)
  * [Rustはこうやって勉強するといいんじゃないか、という一例](https://qiita.com/TakaakiFuruse/items/13e9ad9d1efe7e17811c)
  * [Re:FizzBuzzから始めるRust生活](https://qiita.com/hinastory/items/543ae9749c8bccb9afbc)
  * [Rustを覚えて間もない頃にやってしまいがちなこと](https://qiita.com/mosh/items/709effc9e451b9b8a5f4)
  * [趣味でプログラミングする人がRustを勉強する](https://qiita.com/tonesuke/items/52e469f18e747c794376)
  * [Rust初心者殺しの文法10選](https://qiita.com/muumu/items/8cdcc79fa881912adf51)
  * [Rust のデバッグチートシート](https://qiita.com/legokichi/items/e2f807f70316a916f4be)
  * Tour of Rust まとめ
      * [第1章](https://zenn.dev/kouschatten/articles/tour-of-rust)
      * [第2章](https://zenn.dev/kouschatten/articles/tour-of-rust2)
      * [第3章](https://zenn.dev/kouschatten/articles/tour-of-rust3)
      * [第4章](https://zenn.dev/kouschatten/articles/tour-of-rust4)
      * [第5章](https://zenn.dev/kouschatten/articles/tour-of-rust5)
      * [第6章](https://zenn.dev/kouschatten/articles/tour-of-rust6)

* 文字列
  * [[Rust] 文字列 String から文字列スライス str へ変換される仕組み](https://qiita.com/kerupani129/items/1d170d59720328d7a7c1)
  * [[Rust] 文字列スライス str から文字列 String への変換とその仕組み](https://qiita.com/kerupani129/items/89f31bab79ebba43aeb4)
  * [数字列から一桁ずつ抜き出して処理するやり方](https://qiita.com/butzsuppin/items/178a7564549a9bb166ed)
  * [rust String &str の変換と、文字列 数値 の変換](https://qiita.com/smicle/items/29a4d5d1d14ad7f77f60)
  * [[Rust] &strとStringを理解しようと思ったらsliceやmutを理解できてないことに気づいた話](https://qiita.com/yagince/items/e7474839246ced595f7a)
  * [[Rust] 配列等から文字列のベクタ`Vec<String>`への変換とその仕組み](https://qiita.com/kerupani129/items/37e9e04a47da195267ef)
  * [Rust - strが引数/戻り値で使えない理由について](https://zenn.dev/philomagi/articles/rust_why_cant_use_str_as_both_argument_and_return)
  * [[Rust] to_string メソッド等は呼び出し時に自動参照外しされない](https://qiita.com/kerupani129/items/eaf16e41bc1786a6d049)
  * [[Rust] &str と &&str で呼ばれる to_string メソッドが異なる](https://qiita.com/kerupani129/items/2f61c2f76803c49dede3)
  * [Rustの文字列結合はどうしてString+&strなのか](https://qiita.com/fujiharuka/items/6ef45a3c58ab8153c712)
  * [Rustで文字列の先頭文字や部分文字列を取得する](https://qiita.com/HelloRusk/items/7fb68395984958987a54)
* イテレータ
  * [Rustのイテレータの網羅的かつ大雑把な紹介](https://qiita.com/lo48576/items/34887794c146042aebf1)
  * [Rust のイテレータを使いこなしたい](https://blog-dry.com/entry/2020/06/23/002318)
  * [[Rust] 個人的によく使うイテレーターのメソッドのメモ](https://www.amusement-creators.info/articles/rust/introduce_iterator_methods/)
  * [【Rust】ループとイテレータの使い方](https://tyfkda.github.io/blog/2020/06/13/rust-iter.html)
* コレクション型
  * [Rustで`Vec<T>`に`&`を付けると`&[T]`が得られる理由](https://qiita.com/mosh/items/51bd202c9f738956829e)
  * [[Rust] 配列やベクタが自動でスライスに変換される仕組み](https://qiita.com/kerupani129/items/17584274379f0c65fc34)
  * [Rustでどんな値型も格納できるHashMapを実装する](https://zenn.dev/j5ik2o/articles/21d477b8dbbf70)
  * [Rust で String の Vec を作る](https://zenn.dev/takanori_is/articles/make-string-vec-in-rust)
  * [【Rust】複数要素を持つデータの比較について【then_with()メソッド】](https://qiita.com/AkihiroSasabe/items/8affd44489a2a946e805)
  * [Rustの&strや&[T]はどこを参照しているのか](https://qiita.com/Kogia_sima/items/88920a2a14448ef4dbe3)
  * [メモリをダンプしてRustのsliceとVecを理解する](https://cipepser.hatenablog.com/entry/rust-memory)
* 型
  * [Rustでドメイン固有型を作る際のコツ](https://zenn.dev/j5ik2o/articles/d37bd2c6924446)
  * [Rustで型を強めにつけ、バリデーション情報を型に落とす方法](https://blog-dry.com/entry/2021/07/01/211114)
  * [Rust の型変換イディオム](https://qiita.com/legokichi/items/0f1c592d46a9aaf9a0ea) 
  * [Rustで強めに型をつける](https://keens.github.io/categories/rust%E3%81%A7%E5%BC%B7%E3%82%81%E3%81%AB%E5%9E%8B%E3%82%92%E3%81%A4%E3%81%91%E3%82%8B/)
  * [PhantomDataまとめ](https://qnighy.hatenablog.com/entry/2018/01/14/220000)
  * [Rust の型でスタック操作の正しさを保証する](https://qiita.com/magicant/items/281067cdc6d3eced482f)
  * [Typesafe State in Rust](https://zenn.dev/kinzal/books/aa109c0c428089)
  * [【Rust】SystemTimeをString型の年月日に変換したい](https://zenn.dev/alfina2538/articles/9aa0fe81e424f6)
* パターン
    * [[Rust] 「パターン」を用いた非構造化変数束縛](https://qiita.com/kerupani129/items/f30596eed4e5b2ca7cd1)
    * [Rustでの 抽象化 3パターンについて](https://zenn.dev/j5ik2o/articles/045737392958a3)
    * [Rustのパターンマッチを完全に理解した](https://frozenlib.net/blog/2018-03-11_rust-pattern-match/)
* 構造体
  * [構造体フィールドの所有権の部分借用/移動を理解する](https://qiita.com/yosqueoy/items/453e9aa85bf394388b86)
  * [[Rust] メソッド呼び出し時におけるメソッド探索の仕組み: 自動参照 & 自動参照外し及び Unsized 型強制](https://qiita.com/kerupani129/items/8dba9f5bb2c009c4d08d)
  * [Rustの構造体に文字列を持たせるいくつかの方法](https://qiita.com/Kogia_sima/items/6899c5196813cf231054)
  * [Rustのパブリックなコンストラクタメソッドを持つ構造体とは](https://qiita.com/c3drive/items/0a10cdb82ff3b2eae0e5)
  * [【Rust】可変な構造体に対してgetterを作るべきではない](https://qiita.com/quasardtm/items/d5eae9294fb0e8374aff)
  * [Rustで継承する裏技(トレイトを使わずに)](https://qiita.com/nagisakuya/items/f9a352f279b9ec276819)
* 列挙型
  * [Rust: Enums to wrap multiple errors](https://fettblog.eu/rust-enums-wrapping-errors/)
  * [Rustの列挙型enumの使い方まとめ](https://qiita.com/kujirahand/items/dd655a813a7c2b902f0b)
  * [Rustで`Int(1)`と`Add { id: 1 }`の使い分け](https://zenn.dev/ryo33/articles/a005b562854d46)
* メソッド
    * [Rust の Default::default(), T::new(), T::from() の使い分け](https://qiita.com/syuuu/items/0e4ad366d60d6eb0c7ed)
* トレイト
  * [Rust のポリモルフィズムとトレイトオブジェクト](http://sharply.hatenablog.com/entry/2020/04/19/001236)
  * [Rustの型変換](https://zenn.dev/take4s5i/articles/rust-type-convertion)
  * [Rustのトレイトを使おう!(1)](https://zenn.dev/naughie/articles/0da40abd7bf3cf)
  * [Rustで新しくstructやenum を定義するときに実装を検討するtrait](https://qiita.com/magicant/items/1923d4f8f87a710d97b1)
  * [Rust初心者が自動型変換や型変換関係のトレイトを自信を持って扱えるようになるための型変換まとめ8パターン](https://qiita.com/nirasan/items/e9c621240a7aae914cb8)
  * [[Rust] 引数 &self や戻り値の型 Self に基づくメソッドディスパッチ](https://qiita.com/kerupani129/items/3818b805dfab81ed4da4)
  * [Rustで継承を使いたい人への処方箋](https://qiita.com/muumu/items/a0d111d129d20240d182)
  * [関連型のパターンについて](https://github.com/KeenS/KeenS.github.io/blob/dffbe384cb448cf5409fc35e3e79b36492f0b2ce/content/post/Rustnokanrenkatanotsukaidokoro.md)
  * [[Rust] マーカトレイトから見る言語仕様](https://zenn.dev/senk/articles/0e57e6da138e77)
  * [rustのimpl分岐テクニック](https://qiita.com/wada314/items/12cd69dfd7b02f46b6a6)
  * [トレイト境界の落とし穴](https://zenn.dev/toru3/articles/f27cb26c98cc26)
  * [Rustの`std::convert`の`From`とか`Into`トレイトがわからなくなった時に見る記事](https://qiita.com/SenK/items/b42b4dc95ab979098f12)
  * [Rustのderiveはあまり頭がよくない](https://qnighy.hatenablog.com/entry/2017/06/01/070000)
  * [Rust における`From<T>`とか`Into<T>`とかの考え方](https://qiita.com/hadashiA/items/d0c34a4ba74564337d2f)
  * [`impl Trait`について](https://qnighy.hatenablog.com/entry/2018/01/28/220000)
  * [【Rust】ジェネリック境界わかんないッピ…](https://qiita.com/moyamoyac/items/f49e1ef3ce717d35fee6)
  * [erased_serdeで学ぶobject safeじゃないtraitをtrait objectで使う方法](https://qiita.com/aobat/items/f8499e29d0c38cfe32c8)
  * [Rangeを引数として受け取る](https://qiita.com/nagisakuya/items/21d72ff3c0f66b9fc62c)
  * [`Arc<impl trait>`をスレッドに渡す方法](https://qiita.com/nagisakuya/items/581d232d53cf4b382093)
* 関数ポインタ、クロージャ
    * [Rustにおける関数ポインタやクロージャ](https://zenn.dev/garasubo/articles/97f794cfd469b9d51c7b)
* `Option`型、`Result`型
  * [Rust のエラーハンドリングはシンタックスシュガーが豊富で完全に初見殺しなので自信を持って使えるように整理してみたら完全に理解した](https://qiita.com/nirasan/items/321e7cc42e0e0f238254?utm_content=buffera6970&utm_medium=social&utm_source=twitter.com&utm_campaign=buffer)
  * [thiserrorでカスタムエラーを実装する](https://zenn.dev/hideoka/articles/e2408b1eb8ee3f)
  * [[Rust] Result メソッド早見表](https://qiita.com/kerupani129/items/1e2c5f267081d0dba023)
  * [[Rust] Option メソッド早見表](https://qiita.com/kerupani129/items/a45c614279e7fc58f129)
  * [Rust の Option と Result のメソッド早見表](https://qiita.com/akicho8/items/90545e7e92788198a180)
  * [RustでOptionの配列の最大値・最小値を取り出す](https://zenn.dev/shotaro_tsuji/articles/9c7eedd24b5002)
  * [Rust/AnyhowのTips](https://zenn.dev/yukinarit/articles/b39cd42820f29e)
  * [RustのResultとOptionの変換関数](https://qiita.com/mikiymk001/items/5df69c072472f198c173)

* WEB
  * [Rustとactix_webでWebアプリケーションを作ってみる](https://qiita.com/c3drive/items/71dda219f0193ae72069)
  * [RustでWebアプリケーションを作る](https://caddi.tech/archives/416)
  * [RustでWeb APIを叩く](https://qiita.com/odayushin/items/0e2a5a3d047e6b08c811)
  * [[Rust]reqwestで祝日の取得(json, csv)](https://qiita.com/deepgreenAN/items/f4ac8aa384315675b788)
  * [A Rust web server / frontend setup like it's 2022 (with axum and yew)](https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/)
  * [Rust初心者がRust + actix-web + diesel(MySQL) + serdeでREST APIを作ってみた](https://qiita.com/c0ridrew/items/42bcf381766ba224eed7)
* マルチスレッド
  * [マルチスレッドプログラミングのメモ](https://totem3.hatenablog.jp/entry/2017/05/10/210000)
  * [Rustはどのようにして安全な並列処理を提供するのか](https://qiita.com/nirasan/items/97263103f076bd525a7b)
  * [Multithreading in Rust](https://nickymeuleman.netlify.app/blog/multithreading-rust)
  * [Rustで安全にスレッド間共有変数を利用できることのありがたみと実装例](https://qiita.com/muumu/items/f264ad781486d3dd037b)
* 非同期
  * [Rustの非同期プログラミングをマスターする](https://tech-blog.optim.co.jp/entry/2019/11/08/163000)
  * [非同期 Rust パターン](https://qiita.com/legokichi/items/4f2c09330f90626600a6)
  * [Rust でお気楽非同期プログラミング](https://qiita.com/Kumassy/items/fec47952d70b5073b1b7)
  * [Rust で複数の非同期処理を並行的に実行する](https://qiita.com/yongikim/items/f4a9449b3730048a2398)
  * [Rust のreqwest を使った非同期HTTP Client のお試し](https://qiita.com/mypsychology0/items/60d638ffa8231f4379c0)
  * [Futures Nostalgia](https://fasterthanli.me/articles/futures-nostalgia)
  * [Rustの非同期を浅く整理する](https://zenn.dev/belle/articles/3f3b4fff25c5e9)

* セマンティクス
  * [コピーセマンティクスとムーブセマンティクス](https://opaupafz2.hatenablog.com/entry/2021/07/17/202701)
  * [多コピーの原罪](https://blog.ojisan.io/many-copies-original-sin/)
  * [Rustのcloneはディープコピーなのか](https://zenn.dev/garasubo/articles/aa9a1b5b96e8e9)
* ライフタイム、メモリ関連
  * [Rustのライフタイムについてのよくある誤解](https://github.com/pretzelhammer/rust-blog/blob/master/posts/translations/jp/common-rust-lifetime-misconceptions.md)
  * [Rustのライフタイム注釈について調べてみた](https://blog-mk2.d-yama7.com/2020/12/20201230_rust_lifetime/)
  * [あの日見た匿名ライフタイム](https://zenn.dev/takanori_is/articles/anonymous-lifetime-the-lifetime-we-saw-that-day)
  * [Rustの変数がメモリ上でどのように配置され、moveしたときにどのような挙動をするか](https://qiita.com/iwatake2222/items/79fe0f33fa33d7e2f5c5)
  * [Memory usage of different types for rust structures](https://qiita.com/shooter/items/42a20aae8256e241e924)
  * [Rustで始めるwebフロント開発。フロントエンジニアのためのRustメモリ管理入門](https://zenn.dev/masayannuu/articles/beed577d02dec5)
  * [Rust は何を解決しようとしたのか；メモリとリソースと所有権](https://zenn.dev/karno/articles/630a64fbc9c65e29b566)
  * [【Rust】ライフタイムとVariance](https://qiita.com/maueki/items/b5df36e92561450938dd)
  * [Rustのライフタイムを理解する](https://qiita.com/lechatthecat/items/863198824bbb8c4ab1f4)
  * [[Rust] Lifetime of associated types パターンについて](https://qiita.com/yasuo-ozu/items/40b444dae4404775fe38)
  * [RustのLifetimeってなんなん](https://zenn.dev/ucwork/articles/6de5c9c2257f2d)
  * [RustのOwnershipってなんなん](https://zenn.dev/ucwork/articles/cfe579cbf5647e)
  * [rustの所有権と参照](https://qiita.com/winzu44/items/d40292557ed7e12de77b)
  * [わかる！？Rustの所有権システム](https://zenn.dev/j5ik2o/articles/918c54411d5a61)
  * [Borrow, rules?](https://zenn.dev/toyboot4e/articles/aliasing-rules)
  * [Non-Lexical Lifetimes](https://qiita.com/_EnumHack/items/8b6ecdeb52e69a4ff384)
  * [Rustの所有権(Ownership)について](https://qiita.com/tajihiro/items/e4ef749812acf29620ad)
  * [[Rust] ゲームでありがちなミュータブル参照の問題](https://qiita.com/msakuta/items/450a0d23505126f644af)
  * Rustの参照渡しを使いこなすために
      * [Rustの参照渡しを使いこなすためにその１](https://qiita.com/etnk/items/795f52ffcc6001cb0723)
      * [Rustの参照渡しを使いこなすためにその２](https://qiita.com/etnk/items/eac576506322c3bb3e04)
  * [The “ref” keyword in Rust](https://levelup.gitconnected.com/the-ref-keyword-in-rust-a81e64cda3af)
  * [Rustのメモリ管理って面白い](https://qiita.com/ksato9700/items/312be99d8264b553b193)
  * [Rust のメモリーコンテナー的な何かをわかりやすく整理したチートシートのメモ; T, Cell, RefCell, AtomicT, Mutex, RwLock, Rc, Arc](https://usagi.hatenablog.jp/entry/2020/08/22/040059)
      * [解説](https://qiita.com/usagi/items/fc329895cebd3466910e)
  * [所有権と借用について](https://qiita.com/yz2cm/items/9a8337c368cf055b4255)
  * [Rustの落とし穴の話](https://zenn.dev/mith_mmk/articles/e6483b2b372784)

* コンパイラ
  * [本家Rustコンパイラのソースを読もうとしてみる（1）](https://qiita.com/0yoyoyo/items/eba97a019d0e60324263)
* Future
  * [Rust の Future に入門した](https://zenn.dev/nojima/articles/30bef27473a6fd)
  * [Understanding Rust futures by going way too deep](https://fasterthanli.me/articles/understanding-rust-futures-by-going-way-too-deep)
* ポインタ
  * [Smart Pointers in Rust: What, why and how?](https://dev.to/rogertorres/smart-pointers-in-rust-what-why-and-how-oma)
  * [RustのSizedとfatポインタ](https://qnighy.hatenablog.com/entry/2017/03/04/131311)
  * [Rustの参照、Box、Rcを関数の引数・返り値にした場合の挙動](https://zenn.dev/exyrias/articles/c1d1c6d825fbbb166d44)
  * [RustのSmart pointersってなんなん](https://zenn.dev/ucwork/articles/4fc4cfa47cda26)
  * [Rustの `Arc` を読む(1): Arc/Rcの基本](https://qiita.com/qnighy/items/4bbbb20e71cf4ae527b9)
  * [【Rust勉強】スマートポインタをざっくり](https://qiita.com/moyamoyac/items/5aea471d6676625dcd62)
* ジェネリクス
  * [jeffa.io](https://jeffa.io/)
  * [【Rust】ジェネリクスの取説](https://qiita.com/quasardtm/items/09952838a6ee9582db1d)
* ゼロコスト抽象化
  * [Rustのゼロコスト抽象化の効果をアセンブラで確認](https://blog.rust-jp.rs/tatsuya6502/posts/2019-12-zero-cost-abstraction/)
* マクロ
    * [RustでJSONから値をゆるりと取り出すマクロを書いた話](https://zenn.dev/jiftechnify/articles/rust-macro-for-query-json)
    * [Rustのマクロを覚える](https://qiita.com/k5n/items/758111b12740600cc58f)
    * [Rustの全マクロ種別が分かったつもりになれる話](https://speakerdeck.com/optim/rust-all-kinds-of-macro)
    * [[Rust] Procedural Macroの仕組みと実装方法](https://zenn.dev/tak_iwamoto/articles/890771ea5b8ad3)
* テスト
    * [Rust sqlxでデータベースに依存した部分のテストを書く](https://zenn.dev/htlsne/articles/rust-sqlx-test)
* パフォーマンス
    * [Rust のパフォーマンスに何が影響を与えているのか](https://qiita.com/benki/items/ee14ee6cb9f209a080e1)

* cargo
    * [cargo new でgitを使わない（バージョン管理システムを指定する）方法](https://qiita.com/nobushi95/items/2ce37f6761938efaf339)
* ビルド、CI/CD
    * [[CI/CDでRustビルド高速化](https://qiita.com/pham_thanh_thuong/items/e2c5c1b94a87e941dfff)]
* モジュール
    * [Rustのモジュールの使い方 2018 Edition版](https://keens.github.io/blog/2018/12/08/rustnomoju_runotsukaikata_2018_editionhan/)
    * [Rustのmodule完全に理解した。](https://zenn.dev/newgyu/articles/3b4677b4086768)
    * [【Rust】親子関係にない別ディレクトリにあるmoduleを参照したい](https://zenn.dev/someone7140/articles/4c03867adaa018)

* ベンチマーク
    * [【New SQL実装編】Docker + Rust + PostgreSQL/CockroachDBでベンチマーク](https://qiita.com/murata0531/items/f4ed691b5bfb6af973dc)

* アルゴリズム
    * [パン屋のアルゴリズム](https://zenn.dev/ekusiadadus/articles/qiita_export_bread)
    * [Rustで線形探索と2分探索を書いてみる](https://zenn.dev/utamono832/articles/54a6937966399a)

* 原文翻訳
  * [【翻訳】Rustにおけるパフォーマンスの落とし穴](https://codom.hatenablog.com/entry/2017/06/03/221318)
* This Week in Rust
  * Combining Axum, Hyper, Tonic, and Tower for hybrid web/gRPC apps
    * [Combining Axum, Hyper, Tonic, and Tower for hybrid web/gRPC apps: Part 1](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part1/)
    * [Combining Axum, Hyper, Tonic, and Tower for hybrid web/gRPC apps: Part 2](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part2/)
    * [Combining Axum, Hyper, Tonic, and Tower for hybrid web/gRPC apps: Part 3](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part3/)
    * [Combining Axum, Hyper, Tonic, and Tower for hybrid web/gRPC apps: Part 4](https://www.fpcomplete.com/blog/axum-hyper-tonic-tower-part4/)

  * A Data Pipeline for Go Trains Delay Analysis
      * [A Data Pipeline for Go Trains Delay Analysis — Part 1](https://towardsdatascience.com/a-data-pipeline-for-go-trains-delay-analysis-part-1-21b2c2496aca)
  * Implementing ICMP in Rust
      * [I. Implementing ICMP in Rust](https://dev.to/xphoniex/i-implementing-icmp-in-rust-296o)
      * [II. Implementing ICMP in Rust](https://dev.to/xphoniex/ii-implementing-icmp-in-rust-3bk5)
  * [Polymorphism in Rust](https://oswalt.dev/2021/06/polymorphism-in-rust/)
  * [How to build a job queue with Rust and PostgreSQL](https://kerkour.com/blog/rust-job-queue-with-postgresql/)
  * [Asynchronous Rust: basic concepts](https://dev.to/rogertorres/asynchronous-rust-basic-concepts-44ed)
  * [Rust Option and Result](https://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/)
  * [[Rust] Tokio stack overview: Runtime](https://dev.to/rogertorres/rust-tokio-stack-overview-runtime-9fh)
  * [Rust data structures with circular references](https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/)
  * [Monitoring Rust web application with Prometheus and Grafana](https://romankudryashov.com/blog/2021/11/monitoring-rust-web-application/)
  * [The Java Tutorials with Rust](https://rust-java-tutorials.netlify.app/blog/)
  * [Builder pattern in Rust](https://www.greyblake.com/blog/2021-10-19-builder-pattern-in-rust/)
  * [A Beginner's Guide to Parsing in Rust](https://depth-first.com/articles/2021/12/16/a-beginners-guide-to-parsing-in-rust/)
  * [Thread Safety in C++ and Rust](https://blog.reverberate.org/2021/12/18/thread-safety-cpp-rust.html)
  * [Investigating Memory Allocations in Rust](https://ysantos.com/blog/malloc-in-rust)
  * [Functional Programming in Rust](https://kerkour.com/rust-functional-programming)
  * Procedural macros under the hood
      * [Procedural macros under the hood](https://blog.jetbrains.com/rust/2022/03/18/procedural-macros-under-the-hood-part-i/)
  * [I do understand the * operator in Rust now (updated)](https://micouy.github.io/rust-dereferencing/)
  * [Do we really need language support for self-references?](https://robinmoussu.gitlab.io/blog/post/2022-03-16_do_we_really_need_language_support_for_self_references/)
  * [Learning Rust: My 6 Key Moments](https://apollolabsblog.hashnode.dev/learning-rust-my-6-key-moments)  
  * Pointers Are Complicated
      * [Pointers Are Complicated, or: What's in a Byte?](https://www.ralfj.de/blog/2018/07/24/pointers-and-bytes.html)
      * [Pointers Are Complicated II, or: We need better language specs](https://www.ralfj.de/blog/2020/12/14/provenance.html)
      * [Pointers Are Complicated III, or: Pointer-integer casts exposed](https://www.ralfj.de/blog/2022/04/11/provenance-exposed.html)
  * [Building a crawler in Rust: Associated Types](https://kerkour.com/rust-crawler-associated-types)
  * [Implied bounds and perfect derive](https://smallcultfollowing.com/babysteps/blog/2022/04/12/implied-bounds-and-perfect-derive/)
  * [dyn* doesn't need to be special](https://dev.to/cad97/dyn-doesnt-need-to-be-special-3ldm)
  * [Rust's Unsafe Pointer Types Need An Overhaul](https://gankra.github.io/blah/fix-rust-pointers/)
  * [The Tower of Weakenings: Memory Models For Everyone](https://gankra.github.io/blah/tower-of-weakenings/)
  * [Rust on Nails:A full stack architecture for Rust web applications](https://cloak.software/blog/rust-on-nails/https://cloak.software/blog/rust-on-nails/)
  * [What a better Rust would look like](https://kerkour.com/what-a-better-rust-would-look-like)
  * [Rust traits and dependency injection](https://jmmv.dev/2022/04/rust-traits-and-dependency-injection.html)
  * [The magic of AsRef](https://swatinem.de/blog/magic-asref/)
  * [Request coalescing in async Rust](https://fasterthanli.me/articles/request-coalescing-in-async-rust)
  * [The Better Alternative to Lifetime GATs](https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats)
  * [A shiny future with GATs](https://jackh726.github.io/rust/2022/05/04/a-shiny-future-with-gats.html)
  * [Converting Integers to Floats Using Hyperfocus](https://blog.m-ou.se/floats/)
  * [Parsing / Recursive Descent Parser](https://www.huy.rocks/everyday/05-08-2022-parsing-recursive-descent-parser)
  * [Extending SQLite with Rust](https://ricardoanderegg.com/posts/extending-sqlite-with-rust/)
  * [A Guided Tour of Streams in Rust](https://www.qovery.com/blog/a-guided-tour-of-streams-in-rust)
  * [Let's talk about this async](https://conradludgate.com/posts/async)

* 未分類
  * [Rustのファイルパスの扱いが複雑すぎる件](https://qiita.com/kujirahand/items/b5ab1429b51ab674f5cf)
  * [Rustでお手軽スクレイピング 2020年夏](https://qiita.com/YoshiTheQiita/items/f66828d61293c75a4585)
  * [RustのPlottersでグラフ描画を試す](https://qiita.com/showchan33/items/0b58b598c5e0e7bf1689)
  * [[Rust] フォード・ファルカーソン法を実装する](https://qiita.com/deepgreenAN/items/aa9d8b9d19384fa0a70a)
  * [Rustでログってどう取るの？](https://zenn.dev/belle/articles/900e490ae8dbfe)
  * [日経平均のチャート画像を毎日Twitter投稿するLambda関数（Rust）を作る](https://qiita.com/c3drive/items/59fbdfade4aa2317db83)
  * [BFSをrustで実装](https://qiita.com/butzsuppin/items/4ec9bd61047aba286e95)
  * [RustでBinary treeの left, right のポインターに使うデータ型を考える](https://qiita.com/syuuu/items/11743f61852672c92268)
  * [RustのOptionの as_ref() を堪能できる Hello wold](https://qiita.com/syuuu/items/ac3f72370ee07998be70)
  * [順列の組み合わせ全列挙（再帰関数の理解含む）](https://qiita.com/butzsuppin/items/c32f3d30e3a4e9d4fc55)
  * [【Rust】モダンなKey-Value型データベースSledのテーブルをSerdeとTraitで管理してORMapperを実現する](https://zenn.dev/yosemat/articles/3c281c7d6e073d)
  * [RustでMacで動くCコンパイラを作成する](https://qiita.com/AtsukiTak/items/0819ee57af2639891ecf)
  * [Rustを学ぶための実装：Monte Carlo編](https://qiita.com/Merdane/items/54ea60a0ee9daa56851c)
  * [コンセプトから理解したいRust (願望) Trait編](https://qiita.com/seikoudoku2000/items/28c5c6b09dcaea1744dc)
  * [Rustの std::sync::RwLock はLinuxでwriter starvation問題を起こす (macOSなら平気)](https://laysakura.github.io/2022/03/30/rust-RwLock-writer-starvation/)
  * [RustでGroupBy(ToDictionary)をしたい](https://qiita.com/season3/items/9d404e8b24afa784398e)
  * [RustとPostgreSQLで色々な型をやりとりしてみた(NUMERIC対応)](https://qiita.com/aoyagikouhei/items/d03fcd2c874d42adcab0)

* 他言語比較
  * [Ruby脳のためのRustの配列的なやつの主なメソッド](https://qiita.com/akicho8/items/f68495f5270ba29d45d4)
  * [Ruby脳のためのRustの文字列メソッド](https://qiita.com/akicho8/items/8d4eb552987c1cfd7195)
  * [Rust初心者にとってEnumは難しい！JavaのEnumと何が違うの？](https://qiita.com/Miyukiumoo/items/ec7441cb6783c61e58cf)



---

### framework

- [Rustの新しいWEBフレームワークaxumを触ってみた](https://zenn.dev/techno_tanoc/articles/99e54c82cb049f)
- [Rustの非同期ランタイム `#[tokio::main]`を深堀り](https://qiita.com/ryuma017/items/1f31f5441ed5df80f1cc)

---

### ライブラリ

- [Rustの便利クレート](https://qiita.com/qryxip/items/7c16ab9ef3072c1d7199)
- [RustのコマンドラインライブラリArghの使い方](https://zenn.dev/nak3/articles/76c98cde31aa0d)
- [Rustで手軽にCLIツールを作れるclapを軽く紹介する](https://qiita.com/Tadahiro_Yamamura/items/4ae32347fb4be07ea642)
- [Clap の Derive API で列挙型のコマンドラインオプションを実装する](https://zenn.dev/takanori_is/articles/rust-clap-derive-api-arg-enum)
- [Command line argument parsing in Rust using Clap](https://blog.logrocket.com/command-line-argument-parsing-rust-using-clap/)
- [Rust のフロントエンドフレームワーク Yew の始め方](https://blog1.mammb.com/entry/2022/02/24/232622)
- [Rust GUI / iced 入門](https://zenn.dev/hideakitai/articles/rust_gui_iced_introduction_ht)
- [Rustで良さげなエラーメッセージを出力 w/ anyhow, thiserror](https://zenn.dev/tos_kamiya/articles/71171d0423b1d6)

---

### ロギング

* [Rustのロギングについて解説とデモ](https://www.forcia.com/blog/001605.html)
* [tracing crateを利用したRustのlogging方法について](https://caddi.tech/archives/2144)

---

### crates.io

* [Wasm 対応で Rust 製の JSON <> YAML <> TOML 相互変換 CLI を開発した話](https://zenn.dev/matken/articles/about-jyt-cli)

---

### diesel

* [diesel公式サイト](https://diesel.rs/guides/getting-started/)

---

### other

- [Rust Advent Calendar2020](https://qiita.com/advent-calendar/2020/rust)
- [Rust Advent Calendar2021](https://qiita.com/advent-calendar/2021/rust)
- [cargo-clean-recursive](https://crates.io/crates/cargo-clean-recursive)
- [OPTiM TECH BLOG(Rust)](https://tech-blog.optim.co.jp/archive/category/Rust)
- [CADDi ENGINEER TECH BLOG](https://caddi.tech/archives/category/technology/backend)
- [zenn(rustlang)](https://zenn.dev/topics/rust?order=latest)
- [teratail](https://teratail.com/tags/Rust/active/1)
- [言語処理100本ノック 2015](http://www.cl.ecei.tohoku.ac.jp/nlp100/)
- [Rust で Web バックエンド開発をはじめる](https://developers.cyberagent.co.jp/blog/archives/31110/)
- [Rustハンズオン@エウレカ社](https://speakerdeck.com/helloyuk13/rusthanzuon-at-eurekashe)
- [Rustハンズオン @ Rust CA 1 Day Youth Boot Camp](https://speakerdeck.com/helloyuk13/rusthanzuon-at-rust-ca-1-day-youth-boot-camp)
    - [github](https://github.com/yuk1ty/rust-basic-handson/)
- [Rust ハンズオン@chikoski](https://chikoski.info/rust-handson/)
- [プログラムがメモリをどう使うかを理解する(1)](https://zenn.dev/rita0222/articles/e6ff75245d79b5)
- [基本からしっかり学ぶRust入門(@IT)](https://atmarkit.itmedia.co.jp/ait/series/24844/)
- [Rust勉強会](https://scrapbox.io/nomlab/Rust%E5%8B%89%E5%BC%B7%E4%BC%9A)
- [Rust言語 2021年版の計画（抄訳）](https://zenn.dev/ice_creamer/articles/53c12111ab8d4b)
- [Rustup 1.24.2 について（抄訳）](https://zenn.dev/ice_creamer/articles/a57addc6d5e200)
- [ハーバード大学 プログラミング講座](https://cs50.jp/)
- [一週間で身につくアルゴリズムとデータ構造](http://sevendays-study.com/algorithm/index.html)
- [計算量オーダーについて](https://qiita.com/asksaito/items/59e0d48408f1eab081b5)
- [初学者に教えたい、MicrosoftがGitHubで公開している教材が最高だった！](https://qiita.com/ozora/items/9c801d3b0137eccc32fa)
- [RDBのデータモデリング・テーブル設計の際に参考にしている考え方と資料](https://zenn.dev/rebi/articles/28c7f1fee5730a)

### 個人ブログ
- [κeenのHappy Hacκing Blog](https://keens.github.io/categories/rust/)
- [簡潔なQ](https://qnighy.hatenablog.com/archive/category/Rust)
- [やってみる](https://ytyaru.hatenablog.com/archive/category/Rust)
- [30歳からのプログラミング](https://numb86-tech.hatenablog.com/archive/category/Rust)
