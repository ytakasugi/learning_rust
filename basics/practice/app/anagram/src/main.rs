use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn sorted_string(s: &str) -> String {
    // `char()`は、文字列スライスの文字に対するイテレータを返す
    // `collect()`は、イテレータをコレクションに変換する
    // `Vec<_>`は、`Vec<char>`のことを表している。コードから型が明らかであるため省略記法を使用している
    // 受け取った文字列から文字のイテレーターを取り出し(chars)、Vec<char>型のコレクションに変換している
    let mut s = s.chars().collect::<Vec<_>>();
    // スライスを並び変える
    // `sort()`はミュータブルな参照として`s`を受け取る
    s.sort();
    // `into_iter()`は、`T`を反復処理する(所有権を奪う)
    // `into_iter()を使用して、文字のイテレーターから文字列を作成する
    s.into_iter().collect::<String>()
}

struct Anagram(HashMap<String, Vec<String>>);

impl Anagram {
    // トレイト境界`AsRef<Path>`は、ざっくり意訳すると「パス名っぽいもの」を表す
    // `Self`は、`Anagram`へのエイリアス
    fn new<P: AsRef<Path>>(dictfile: P) -> Result<Self, io::Error> {
        // ファイルを開く
        let file = File::open(dictfile)?;
        let file = io::BufReader::new(file);
        // ハッシュマップを準備しておく
        let mut anagram = Anagram(HashMap::new());
        for line in file.lines() {
            let word = line?;
            anagram.add_word(word);
        }
        Ok(anagram)
    }

    // テーブルを更新するので`&mut self`を使う
    // 登録した単語をテーブルが所有するので、`word`の所有権も奪う
    fn add_word(&mut self, word: String) {
        // 単語をアルファベット順にソートしたものを作ってキーにする
        let sorted = sorted_string(&word);
        // キーに対応する値があればそれを、なければ新たにデフォルト値（Vec::new()）を入れる
        // 返り値はキーに対応する値
        // ハッシュマップはデータの所有者なので、キーもデフォルト値も所有権を奪う
        self.0.entry(sorted).or_insert(Vec::new()).push(word);

    }

    // 検索はリードオンリーなので`&self`を使う
    // キーはリードオンリーなので`word`も参照で受け取る
    fn find(&self, word: &str) -> Option<&Vec<String>> {
        let word = sorted_string(word);
        // データの所有権はハッシュマップにあるので、返り値は参照型
        // 参照型なのでコピーは発生せず、高速
        self.0.get(&word)
    }
}

fn main() {
    let word = std::env::args().nth(1).expect("Usage: word");
    let table = Anagram::new("./dict/words")
                .expect("failed to make table");
    println!("{:?}", table.find(&word));
}