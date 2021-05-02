### [リテラルとエスケープ](https://doc.rust-jp.rs/rust-by-example-ja/std/str.html#literals-and-escapes)

特殊文字を含む文字列リテラルの書き方は複数あります。どれも似たような`&str`になるので、書きやすい形式を使うのが一番です。同様に、バイト文字列リテラルにも複数の書き方がありますが、いずれも`&[u8; N]`となります。

一般的に特殊文字はバックスラッシュでエスケープされます。\. この方法では、印刷できない文字や入力方法がわからない文字も含めて、あらゆる文字を文字列に追加することができます。文字通りのバックスラッシュが必要な場合は、別のバックスラッシュでエスケープします。`\\`

リテラルの中にある文字列や文字リテラルの区切り文字はエスケープしなければなりません。`"\""`, `'\''`.

~~~rust
fn main() {
    // エスケープを使ってバイトを16進数で書くこともできますが...。
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...またはUnicodeコードポイントでバイトを記述することができます。
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );


    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}

~~~

エスケープしなければならない文字が多すぎる場合や、文字列をそのまま書いた方が便利な場合もあります。そんなときは、生の文字列リテラルが活躍します。

~~~rust
fn main() {
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // 生の文字列に引用符が必要な場合は、#sのペアを追加します。
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // 文字列の中に「#」が必要な場合は、デリミタに#を多めに入れます。
    // 使用できる#の数に制限はありません。
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);
}
~~~

UTF-8ではない文字列が必要ですか？strとStringは有効なUTF-8でなければなりません）。あるいは、ほとんどがテキストであるバイト配列が欲しいのかもしれません。バイト文字列の出番です。

~~~rust
use std::str;

fn main() {
    // これは実際には `&str` ではないことに注意してください。
    let bytestring: &[u8; 21] = b"this is a byte string";

    // バイト配列には `Display` という特性がないので、printするには少し制限があります。
    println!("A byte string: {:?}", bytestring);

    // バイト文字列は、バイトエスケープを持つことができます...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...しかし、ユニコードエスケープはありません
    // `let escaped = b"\u{211D}`は許されない;
    println!("Some escaped bytes: {:?}", escaped);


    // 生のバイト文字列は、生の文字列と同様に動作します。
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // バイト配列の `str` への変換に失敗することがある
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // バイト文字列はUTF-8である必要はありません。
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // "ようこそ" in SHIFT-JIS

    // しかし、その場合、常に `str` に変換できるとは限りません。
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };
}
~~~

文字エンコーディング間の変換については、[encoding](https://crates.io/crates/encoding)クレートをご覧ください。

文字列リテラルやエスケープ文字の書き方については、『Rust Reference』の[Token](https://doc.rust-lang.org/reference/tokens.html)の章で詳しく説明されています。