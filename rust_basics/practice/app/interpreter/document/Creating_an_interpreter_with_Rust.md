# はじめに

* [Go言語でつくるインタプリタ](https://www.amazon.co.jp/dp/4873118220/ref=cm_sw_em_r_mt_dp_U_4CCYCbJ6DQPXJ)が面白かったので、書籍の内容の復習と Rust の学習のために Rust で四則演算インタプリタをつくってみました。


# インタプリタとは

* インタプリタとはソースコードが入力されるとプログラムとして実行するプログラムのことです。
* インタプリタがソースコードを読み込んでからプログラムとして実行するまでの間には `字句解析`、`構文解析`、`実行` という三つの工程があります。


## 字句解析とは

* 字句解析はインタプリタが行う最初の工程です。
* 字句解析の目的はソースコードを正規化してプログラムの部品として扱いやすくすることです。
* 字句解析を行うプログラムを字句解析器と呼びます。
* 字句解析器は文字列であるソースコードを入力すると、プログラムの最小単位である `字句` の配列を出力します。

```:字句解析器の入出力例
# 字句解析の入力（ソースコード）
"1 + 2 * 3"

# 字句解析の出力（字句の配列）
[ "1", "+", "2", "*", "3" ]
```


## 構文解析とは

* 構文解析はインタプリタが行うふたつ目の工程です。
* 構文解析の目的は正規化されたソースコードをプログラムの構文として解釈して実行可能なデータ形式に変換することです。
* 構文解析を行うプログラムを構文解析器と呼びます。
* 構文解析器は字句解析器から出力された字句の配列を入力すると、プログラムとして実行可能な `構文木` という構造のデータとして出力します。

```:構文解析器の入出力例
# 構文解析の入力（字句の配列）
[ "1", "+", "2", "*", "3" ]

# 構文解析の出力（構文木）
{ 左辺: 1, 演算子: +, 右辺: { 左辺: 2, 演算子: *, 右辺: 3 } }
```

* 構文木は分岐点に演算子、葉に演算子の処理対象の式を持つ木構造のデータです。
* 構文木は実行可能なデータ形式で、実行時には木の末端から実行されます。
* そのため、構文解析器は優先度の高い演算子の式ほど末端に配置されるような形で構文木を作成します。
* 先の例の構文木を図にすると以下のようになります。加算より乗算のほうが優先度が高いので、加算より乗算のほうが深い位置に配置されます。

```:構文木の図
"+" ─ "1"
    └ "*" ─ "2"
          └ "3"
```


## 実行とは

* 実行はインタプリタが行う最後の工程です。
* 実行では構文木を入力としてプログラムを実行します。
* 今回実装するインタプリタは四則演算を行うだけのものなので、実行により行われた計算結果を常に出力します。

```:実行の入出力例
# 実行の入力（構文木）
{ 左辺: 1, 演算子: +, 右辺: { 左辺: 2, 演算子: *, 右辺: 3 } }

# 実行の出力（実行結果）
7
```


# 字句解析器をつくる

* 字句解析器は文字列であるソースコードを入力すると、プログラムの最小単位である `字句` の配列を出力するプログラムです。
* 字句解析器の行う処理は単純で、入力されたソースコードの文字列を先頭から順に読み込んでいき、空白の除去などの正規化をしながら文字を字句（Token）に置き換えます。

## 字句を定義する

* まずは字句解析器が出力する字句を `Token` 列挙体として定義します。
* 今回は単純な四則演算だけできればいいので字句としては `数字`, `+`, `-`, `*`, `/`, `(`, `)` に相当するものを定義します。

```rust
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64), // 数字
    Plus, // +
    Minus, // -
    Asterisk, // *
    Slash, // /
    LParen, // (
    RParen, // )
}
```

## 字句解析器を実装する

* 字句解析器は構造体 `Lexer` として定義します。
* Lexer は初期化時にソースコードの文字列を受け取ります。
* 字句解析の実行は `Lexer::token` 関数で行います。
* Lexer::token を実行すると文字列の先頭から順に文字を読み込み、ひとつの字句に変換できたら字句を返して終了します。文字列の終端まで読み込み終わった場合 None を返します。
* `+`, `-` などの演算子のようにひとつの文字にひとつの字句が対応する場合には変換は単純です。
* `10`, `2.5` などの数字の場合はひとつ以上の文字にひとつの字句が対応するので少し複雑になります。文字の長さは決まっていないので、現在解析中の文字の次の文字が数字であるかどうかを確認し、数字であれば字句として取り込むという流れになります。

```rust
struct Lexer {
    // 入力された文字列
    input: Vec<char>,
    // 解析中のインデックス
    position: usize,
}

impl Lexer {
    // 初期化
    fn new(input: Vec<char>) -> Lexer {
        Lexer { input, position: 0 }
    }
    // 現在解析中の文字を字句として取得し、インデックスを一つ進める
    fn token(&mut self) -> Option<Token> {
        use std::iter::FromIterator;
        // 空白をスキップする
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }
        // 現在解析中の文字を取得して字句に変換する
        let curr = self.curr()?;
        let token = if Self::is_number(curr) {
            // 数字の場合
            let mut number = vec![*curr];
            while self.peek().is_some() && Self::is_number(self.peek().unwrap()) {
                self.next();
                number.push(*self.curr().unwrap());
            }
            String::from_iter(number)
                .parse::<f64>()
                .ok()
                .and_then(|n| Some(Token::Number(n)))
        } else {
            // 数字以外の場合
            match curr {
                &'+' => Some(Token::Plus),
                &'-' => Some(Token::Minus),
                &'*' => Some(Token::Asterisk),
                &'/' => Some(Token::Slash),
                &'(' => Some(Token::LParen),
                &')' => Some(Token::RParen),
                _ => None,
            }
        };
        self.next();
        return token;
    }
    // 入力された文字列の解析するインデックスをひとつ進める
    fn next(&mut self) {
        self.position += 1;
    }
    // 現在解析中の文字
    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }
    // 次に解析する文字
    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }
    // 文字が数字であるかどうか
    fn is_number(c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}
```

## 字句解析器を実行する

* 実行方法の確認程度に極々簡単にテストを実装します。

```rust
#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("1 + 2".chars().collect());
    assert_eq!(lexer.token(), Some(Token::Number(1_f64)));
    assert_eq!(lexer.token(), Some(Token::Plus));
    assert_eq!(lexer.token(), Some(Token::Number(2_f64)));
    assert_eq!(lexer.token(), None);
}
```


# 構文解析器をつくる

* 構文解析器は正規化されたソースコードをプログラムの構文として解釈し、構文木と呼ばれる実行可能なデータ形式に変換するプロブラムです。

## 構文木を定義する

* まずは構文解析器が出力する構文木を定義します。
* 今回実装するインタプリタはひとつの計算式を受け取ってすぐに計算して結果を出力するというものにするので、式と数字だけ定義します。
* 式には、式の前に演算子のある `前置演算子式` と、式と式の間に演算子のある `中置演算子式` があります。

```rust
enum Expr {
    // 数字
    Number(f64),
    // 前置演算子式
    // 式の前に演算子のついた式
    // 前置演算子は "-" だけ
    // 例）"-10", "-(1 + 2)"
    PrefixExpr {
        operator: String,
        right: Box<Expr>,
    },
    // 中置演算子式
    // 式と式の間に演算子のある式
    // 例）"1 + 2", "3 * (4 + 5 + 6)"
    InfixExpr {
        // 左辺
        left: Box<Expr>,
        // 演算子
        operator: String,
        // 右辺
        right: Box<Expr>,
    },
}
```

## 構文解析器の入出力を確認する

* 構文解析器を実装する前に、構文解析器はどのような字句を入力するとどのような構文木を出力するのかをみていきます。

```
/* 単純な中置演算子式 */

# 入力
["1", "+", "2"]

# 出力
InfixExpr {
  left: Number(1),
  operator: "Plus",
  right: Number(2),
}


/* 中置演算子式と前置演算子式の組み合わせ */

# 入力
["1", "*", "-", "2"]

# 出力
InfixExpr {
  left: Number(1),
  operator: "Plus",
  right: PrefixExpr {
    operator: "Minus", 
    right: Number(2),
  }
}


/* 同じ優先度の演算子が複数ある場合 */
/* 中置演算子式を木構造と見立てるとより深い位置の式から順に計算されます */
/* 同じ優先度の演算子が連なっている場合は左側から順に計算されるので、左側の式を左辺として右側に連なる式を右辺としてどんどん追加していく形で構文木を作っていきます */

# 入力
["1", "*", "2", "*", "3", "*", "4"]

# 出力
InfixExpr {
  left: InfixExpr {
    left: InfixExpr {
      left: Number(1),
      operator: "Asterisk",
      right: Number(2),
    },
    operator: "Asterisk",
    right: Number(3),
  },
  operator: "Asterisk",
  right: Number(4),
}


/* 異なる優先度の演算子が複数ある場合 */
/* "+" より "*" の優先度のほうが大きいので、優先度の高い "2 * 3" が先に計算されるように深い位置に配置されます */

# 入力
["1", "+", "2", "*", "3"]

# 出力
InfixExpr {
  left: Number(1),
  operator: "Plus",
  right: InfixExpr {
    left: Number(2),
    operator: "Asterisk",
    right: Number(3),
  }
}
```

## 構文解析器を定義する

* 構文解析器は構造体 `Parser` として定義します。
* Parser は初期化時に Lexer を受け取り、Lexer の出力する字句を先頭から順に解析して構文木を作成します。

```rust
struct Parser {
    // 字句解析器
    lexer: Lexer,
    // 現在解析中の字句
    curr: Option<Token>,
    // 次に解析する字句
    peek: Option<Token>,
}

impl Parser {
    // 初期化
    fn new(mut lexer: Lexer) -> Parser {
        let curr = lexer.token();
        let peek = lexer.token();
        Parser { lexer, curr, peek }
    }
    // 次の字句を解析対象にする
    fn next(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.token();
    }
}
```

## 構文解析器で数字と前置演算子式を解析する

* Parser で構文木の葉の要素である数字と前置演算子式を解析する関数 `Parser::parse_prefix` について解説します。
* parse_prefix は数字や前置演算子式、`(` と `)` によってグループ化された式を解析するための関数です。
* グループ化された式の解析については後で解説します。
* 数字の解析は `Token::Number` を `Expr::Number` に変換するだけの単純なものです。
* 前置演算子式の解析は、葉の位置の字句が `Token::Minus` である場合に行われます。前置演算子式の解析では "-" を演算子とし、右辺を `Parser::parse_expression` で解析します。Parser::parse_expression の詳細は後で解説しますが、ここでは前置演算子式の右辺には任意の式が当てはまるとだけ理解すれば大丈夫です。

```rust
impl Parser {
    // 構文木の葉の要素の解析
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr.as_ref()? {
            Token::Minus => self.parse_minus(),
            Token::Number(_) => self.parse_number(),
            Token::LParen => self.parse_grouped_expression(),
            _ => None,
        }
    }
    // 前置演算子式の解析
    fn parse_minus(&mut self) -> Option<Box<Expr>> {
        self.next();
        let number = self.parse_expression(Precedence::PREFIX)?;
        return Some(Box::new(Expr::PrefixExpr {
            operator: "Minus".to_string(),
            right: number,
        }));
    }
    // 数字の解析
    fn parse_number(&mut self) -> Option<Box<Expr>> {
        match self.curr.borrow() {
            Some(Token::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }
}
```

## 構文解析器で演算子の優先度を定義する

* 次に中置演算子式の解析について解説していきたいのですが、その準備として中置演算子の優先度を定義していきます。
* 優先度は `Precedence` 列挙体として定義します。優先度の比較を行うため `#[derive(PartialOrd, PartialEq)]` を指定します。
* 構文解析器では `Parser::token_precedence` 関数で演算子の字句から優先度を導き出して演算子同士の優先度の比較を行います。

```rust
#[derive(PartialOrd, PartialEq)]
enum Precedence {
    // 最低
    LOWEST,
    // "+", "-"
    SUM,
    // "*", "/"
    PRODUCT,
    // 前置演算子
    PREFIX,
}

impl Parser {
    fn token_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Slash | Token::Asterisk => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }
}
```

## 構文解析器ですべての種類の式を解析する

* 中置演算子式を含むすべての式の解析は `Parser::parse` 関数で行います。
* まずは必要になるコードをすべて記載し、あとからひとつづつ解説していきます。

```rust
impl Parser {
    fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expression(Precedence::LOWEST)
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        let mut left = self.parse_prefix()?;

        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            left = self.parse_infix(left)?;
        }

        return Some(left);
    }
    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        match token {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }
    pub fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        let operator = format!("{:?}", token);
        let precedence = Self::token_precedence(token);
        self.next();
        let right = self.parse_expression(precedence)?;
        return Some(Box::new(Expr::InfixExpr {
            left,
            operator,
            right,
        }));
    }
    fn is_peek(&self, token: &Token) -> bool {
        if self.peek.is_none() {
            return false;
        }
        mem::discriminant(self.peek.as_ref().unwrap()) == mem::discriminant(token)
    }
    fn peek_precedence(&self) -> Precedence {
        let token = self.peek.borrow();
        if token.is_none() {
            return Precedence::LOWEST;
        }
        return Self::token_precedence(token.as_ref().unwrap());
    }
}
```

### Parser::parser_expression で式を解析する

* `Parser::parse_expression` はすべての種類の式の解析を行います。
* parse_expression は引数に「解析の基準となる優先度」を受け取ります。
* parse_expression はまず、先頭の字句を構文木の葉の要素とみなして `parser_prefix` 関数で解析し左辺とします。
* そのあとに、次の字句を演算子とみなして先読みします。この演算子の優先度が「解析の基準となる優先度」よりも大きい場合、`parse_infix` 関数で中置演算子式として解析を行います。

```rust
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        // (1). 葉の要素の解析
        let mut left = self.parse_prefix()?;

        // (2). 優先度が大きければ中置演算子式の解析を繰り返す
        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            left = self.parse_infix(left)?;
        }

        // (3). 解析した式を返す
        return Some(left);
    }
```

* 中置演算子式の解析は演算子の優先度が「解析の基準となる優先度」よりも大きい限り繰り返し実行されます。
* `parse` で行っているように「解析の基準となる優先度」に `Precedence::LOWEST` という最低の優先度を指定すると、常に字句側の演算子の優先度が高くなるため、すべての字句の解析を行うことができます。

```rust
    fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expression(Precedence::LOWEST)
    }
```

### Parser::parse_infix で中置演算子式を解析する

* `Parser::parse_infix` は演算子が中置演算子である場合に `Parser::parse_infix_expression` を実行して中置演算子式の解析を行います。

```rs
    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        match token {
            Token::Plus | Token::Minus | Token::Asterisk | Token::Slash => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }
```

* `Parser::parse_infix_expression` は中置演算子式の解析を行います。
* parse_infix_expression は引数で左辺の式を受け取ります。
* parse_infix_expression は現在解析中の演算子の優先度で `parse_expression` を実行して右辺の式を取得します。
* **現在の演算子より次の演算子の優先度が大きい場合**、次の演算子式が右辺に取り込まれます。この挙動により**優先度の大きい式が構文木の深い位置に配置される**ようになります。
* **現在の演算子より次の演算子の優先度が小さいか同じ場合**、右辺の先頭の項だけが右辺の式として扱われます。
* parse_infix_expression は左辺の式・演算子・右辺の式を使って中置演算子式を作成して返します

```rust
    fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        let operator = format!("{:?}", token);
        let precedence = Self::token_precedence(token);
        self.next();
        let right = self.parse_expression(precedence)?;
        return Some(Box::new(Expr::InfixExpr {
            left,
            operator,
            right,
        }));
    }
```

## 具体例つきの中置演算子式の構文解析の解説

* 説明だけだと分かりづらいので具体例をあげながら中置演算子式の構文解析の挙動を解説していきます。

### "1 + 2 + 3" を構文解析する場合

```
1. parse で構文解析を開始します

  関数: parse
  字句: 1 + 2 + 3

2. parse_expression で左辺の式を取得します

  関数: parse > parse_expression
  字句: + 2 + 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)

3. 基準となる優先度より現在の演算子の優先度が大きいので parse_infix による中置演算子式の構文解析を開始します

  関数: parse > parse_expression > parse_infix
  字句: + 2 + 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)

4. parse_infix で現在の演算子が中置演算子なので parse_infix_expression による中置演算子式の構文解析を開始します

  関数: parse > parse_expression > parse_infix > parse_infix_expression
  字句: + 2 + 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)

5. 中置演算子式の演算子として字句の先頭の演算子を読み込みます

  関数: parse > parse_expression > parse_infix > parse_infix_expression
  字句: 2 + 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)
  演算子: Plus

6. 中置演算子式の演算子の優先度で parse_expression を実行して右辺の式の取得を開始します

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression
  字句: 2 + 3
  基準となる優先度: Precedence::LOWEST > Precedence::SUM
  左辺の式: Number(1)
  演算子: Plus

7. 字句の先頭を右辺解析中の左辺の式として取得します

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression
  字句: + 3
  基準となる優先度: Precedence::LOWEST > Precedence::SUM
  左辺の式: Number(1)
  演算子: Plus
  右辺解析中の左辺の式: Number(2)

8. 現在の演算子 + の優先度が基準となる優先度より大きくないので parse_expression の処理を終了し、右辺解析中の左辺の値を parse_infix_expression に返します

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression
  字句: + 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)
  演算子: Plus
  返り値: Number(2)

9. InfixExpr を作成して parse_expression に返します

  関数: parse > parse_expression > parse_infix > parse_infix_expression
  字句: + 3
  基準となる優先度: Precedence::LOWEST 
  返り値: InfixExpr { left: Number(1), operator: Plus, right: Number(2) }

10. 返り値を左辺の式とします

  関数: parse > parse_expression
  字句: + 3
  基準となる優先度: Precedence::LOWEST 
  左辺の式: InfixExpr { left: Number(1), operator: Plus, right: Number(2) }

11. 基準となる優先度より現在の演算子の優先度のほうが大きいので、もう一度中置演算子式の解析を行います

  関数: parse > parse_expression
  字句: + 3
  基準となる優先度: Precedence::LOWEST 
  左辺の式: InfixExpr { left: Number(1), operator: Plus, right: Number(2) }

12. 3 から 10 と同様に処理を行い左辺の式を更新します 

  関数: parse > parse_expression
  字句: 空
  基準となる優先度: Precedence::LOWEST 
  左辺の式: InfixExpr { left: InfixExpr { left: Number(1), operator: Plus, right: Number(2) }, operator: Plus, right: Number(3) }

13. 字句が空なので処理を終了して式を返します

  返り値: InfixExpr { left: InfixExpr { left: Number(1), operator: Plus, right: Number(2) }, operator: Plus, right: Number(3) }
```

### "1 + 2 * 3" を構文解析する場合

```
1. parse で構文解析を開始します

  関数: parse
  字句: 1 + 2 * 3

2. parse_expression で左辺の式を取得します

  関数: parse > parse_expression
  字句: + 2 * 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)

3. 基準となる優先度より現在の演算子の優先度が大きいので parse_infix による中置演算子式の構文解析を開始します

  関数: parse > parse_expression > parse_infix
  字句: + 2 * 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)

4. parse_infix で現在の演算子が中置演算子なので parse_infix_expression による中置演算子式の構文解析を開始します

  関数: parse > parse_expression > parse_infix > parse_infix_expression
  字句: + 2 * 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)

5. 中置演算子式の演算子として字句の先頭の演算子を読み込みます

  関数: parse > parse_expression > parse_infix > parse_infix_expression
  字句: 2 * 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)
  演算子: Plus

6. 中置演算子式の演算子の優先度で parse_expression を実行して右辺の式の取得を開始します

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression
  字句: 2 * 3
  基準となる優先度: Precedence::LOWEST > Precedence::SUM
  左辺の式: Number(1)
  演算子: Plus

7. 字句の先頭を右辺解析中の左辺の式として取得します

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression
  字句: * 3
  基準となる優先度: Precedence::LOWEST > Precedence::SUM
  左辺の式: Number(1)
  演算子: Plus
  右辺解析中の左辺の式: Number(2)

8. 現在の演算子 * の優先度が基準となる優先度より大きいので parse_infix, parse_infix_expression を実行します

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression
  字句: * 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)
  演算子: Plus
  右辺解析中の左辺の式: Number(2)

9. 中置演算子式の演算子として字句の先頭の演算子を読み込みます

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression > parse_infix > parse_infix_expression
  字句: 3
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)
  演算子: Plus
  右辺解析中の左辺の式: Number(2)
  右辺解析中の演算子: Asterisk

10. parse_expression を実行して右辺の式を取得します

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression > parse_infix > parse_infix_expression
  字句: 空
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)
  演算子: Plus
  右辺解析中の左辺の式: Number(2)
  右辺解析中の演算子: Asterisk
  右辺解析中の右辺の式: Number(3)

11. InfixExpr を作成して parse_infix_expression に返します

  関数: parse > parse_expression > parse_infix > parse_infix_expression > parse_expression > parse_infix > parse_infix_expression
  字句: 空
  基準となる優先度: Precedence::LOWEST
  左辺の式: Number(1)
  演算子: Plus
  返り値: InfixExpr { left: Number(2), operator: Asterisk, right: Number(3) }

12. 返り値を右辺の式として InfixExpr を作成して parse_expression に返します

  関数: parse > parse_expression > parse_infix > parse_infix_expression
  字句: 空
  基準となる優先度: Precedence::LOWEST
  返り値: InifxExpr { left: Number(1), operator: Plus, right: InfixExpr { left: Number(2), operator: Asterisk, right: Number(3) } }

13. 字句が空なので処理を終了して式を返します

  返り値: InfixExpr { left: InfixExpr { left: Number(1), operator: Plus, right: Number(2) }, operator: Plus, right: Number(3) }
```

## グループ化された式の構文解析

* 最後にグループ化された式の構文解析の解説を行います。
* グループ化された式とは `(` と `)` によって囲まれた式のことで、最も優先して処理される式になります。
* グループ化された式の構文解析は parse_prefix で字句が `(` だった場合に parse_grouped_expression によって行われます。
* parse_grouped_expression は `(` をスキップして parse_expression を実行します。
* parse_expression は字句を読み込んで式を作成してゆき `)` に到達すると処理を終了して作成した式を返します。
* parse_expression が `)` に到達した際に処理を終わらせるのは parse_infix で `)` が中置演算子として定義されていないからです。
* その後 parse_grouped_expression は `)` をスキップして葉の要素としてグループ化された式を返します。

```rust
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr.as_ref()? {
            Token::Minus => self.parse_minus(),
            Token::Number(_) => self.parse_number(),
            Token::LParen => self.parse_grouped_expression(),
            _ => None,
        }
    }
    fn parse_grouped_expression(&mut self) -> Option<Box<Expr>> {
        self.next();
        let expression = self.parse_expression(Precedence::LOWEST);
        if self.is_peek(&Token::RParen) {
            self.next();
            return expression;
        } else {
            return None;
        }
    }
```


## 構文解析器を実行する

* 実行方法の確認程度に極々簡単にテストを実装します。

```rust
#[test]
fn test_parser() {
    do_parser(
        "1 + 2",
        r#"Some(InfixExpr { left: Number(1.0), operator: "Plus", right: Number(2.0) })"#,
    );
    do_parser("- 1 + 2 * 3",
             r#"Some(InfixExpr { left: PrefixExpr { operator: "Minus", right: Number(1.0) }, operator: "Plus", right: InfixExpr { left: Number(2.0), operator: "Asterisk", right: Number(3.0) } })"#);
}

#[cfg(test)]
fn do_parser(input: &str, expect: &str) {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    assert_eq!(format!("{:?}", parser.parse()), expect);
}
```

# 構文木を実行する

* `eval` 関数で構文木を実行します。
* eval は構文木をパターンマッチで再帰的に評価して実行するための極単純な実装になります。

```rust
fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::PrefixExpr { operator: _, right } => -eval(right),
        Expr::InfixExpr {
            left,
            operator,
            right,
        } => {
            let left = eval(left);
            let right = eval(right);
            match operator.as_str() {
                "Plus" => left + right,
                "Minus" => left - right,
                "Asterisk" => left * right,
                "Slash" => left / right,
                _ => panic!("invalid operator"),
            }
        }
    }
}
```

* eval の実行はこのように行います。

```rust
#[test]
fn test_eval() {
    do_eval("1 + 2", 3_f64);
    do_eval("1 + 2 * 3", 7_f64);
    do_eval("1 + (2 + 3) * -(3 / 3)", -4_f64);
}

#[cfg(test)]
fn do_eval(input: &str, expect: f64) {
    let lexer = Lexer::new(input.chars().collect());
    let mut parser = Parser::new(lexer);
    let result = eval(parser.parse().unwrap().borrow());
    assert_eq!(result, expect);
}
```

# インタプリタとして実行する

* 実装したインタプリタをコマンドとして実行できるように main 関数を実装します。
* main 関数は無限ループで標準入力を待ち受けて、入力があったら文字列として字句解析し、構文解析し、実行するというものです。
* `exit` と入力された場合はコマンドを終了するようになっています。

```rust
fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .ok()
            .expect("failed to read line");

        if code == "exit\n" {
            break;
        }

        let lexer = Lexer::new(code.chars().collect());
        let mut parser = Parser::new(lexer);

        let expr = parser.parse();

        if let Some(expr) = expr {
            println!("{}", eval(expr.borrow()));
        }
    }
}
```

# おわりに

* 紹介したコードは[こちら](https://gist.github.com/nirasan/38076bcca46f298ffb3a0f145518e9c4)にアップロードしてあります。
* 今回は簡単のために四則演算を行うだけのインタプリタを実装しましたが、ユーザー定義関数の実装や関数呼び出し、変数定義などの作り方も面白かったので、興味があれば[Go言語でつくるインタプリタ](https://www.amazon.co.jp/dp/4873118220/ref=cm_sw_em_r_mt_dp_U_4CCYCbJ6DQPXJ)も読んでみることをおすすめします。

