use std::error::Error;
use std::fs::File;
use std::env;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argments.");
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        // env::var関数は、環境変数がセットされていたら、環境変数の値を含むOk列挙子の成功値になるResultを返す。 
        // 環境変数がセットされていなければ、Err列挙子を返す
        // is_errメソッドを使用して、エラー、つまり環境変数がセットされていないことを確認している
        // これは、大文字小文字を区別して検索することを意味している
        // `CASE_INSENSITIVE`の値をチェックでなく、環境変数がセットされているかどうかだけを確認している
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{ query, filename, case_sensitive })

    }
}

// 引数にConfigインスタンスを取る
// Resultが`Ok`の場合は`ユニット型()`、エラーの場合は`Error`トレイトを返す
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    // 空のString型を作成
    let mut contents = String::new();
    // すべてのバイトを読み込みバッファに追加する
    f.read_to_string(&mut contents)?;

    // case_sensitiveがtrue(env::varの結果がErr(環境変数がセットされていない)かつis_err結果がtrue)ならsearch関数を実行
    // そうでなければ、search_case_insensitiveを実行
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // search関数に`config.query`と`contents`の参照を引数として渡す
    for line in results {
        println!("{}", line);
    }

    //`ユニット型()`を`Ok()`で包む
    Ok(())
} 

// 空のベクタを返す関数
// search関数に返される値は、search関数にcontents引数で渡されているデータと同じライフタイムを持つ
// スライスに参照されるデータは、参照が有効になるために有効である必要がある
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 可変なベクタを生成し、forループ内でpushを呼び出すことで、line(クエリ文字列を含む行)をベクタへ保存する
    let mut results = Vec::new();
    // linesメソッドは、イテレータを返す
    for line in contents.lines() {
        // クエリを求めて各行を検索する
        if line.contains(query) {
            // 合致した行を保存する
            results.push(line);
        }
    }
    results
}

// search関数とほぼ同じ。 唯一の違いは、queryと各lineを小文字化していること。
//  入力引数の大文字小文字によらず、行がクエリを含んでいるか確認する際には、同じになる。
// search関数と同様に、search_case_insensitive関数に返される値は、search_case_insensitive関数にcontents引数で渡されているデータと
// 同じ値をライフタイムを持っている
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // queryは、文字列スライスではなくStringであることに注意。 to_lowercaseを呼び出すと、既存のデータを参照するのではなく、新しいデータを作成するからです。
    // 例として、クエリは"rUsT"だとする。その文字列スライスは、小文字のuやtを使えるように含んでいないので、 "rust"を含む新しいStringのメモリを確保しなければならない。
    // containsメソッドに引数としてqueryを渡すときに、`&`記号を追加する必要がある。なぜなら、containsのシグニチャは、文字列スライスを取るよう定義されているから。
    // containsのシグニチャは以下の通り
    // pub fn contains<'a, P>(&'a self, pat: P) -> bool
    // where
    //     P: Pattern<'a>, 
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}