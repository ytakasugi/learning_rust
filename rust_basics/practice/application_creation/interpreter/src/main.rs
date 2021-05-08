use std::iter::FromIterator;
use core::borrow::Borrow;
use std::io::{self, Write};
use std::mem;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}

struct Lexer {
    // 入力された文字列
    input: Vec<char>,
    // 解析中のインデックス
    position: usize,
}

// `Lexer`にメソッドを実装
impl Lexer {
    // 初期化
    fn new(input: Vec<char>) -> Lexer {
        Lexer{ input, position: 0 }
    }

    // 解析中の文字を字句として取得し、インデックスを一つ進める
    fn token(&mut self) -> Option<Token> {
        // 空白をスキップする
        // 解析中の要素が文字だった場合、且つ`White_space`プロパティを持っている場合、インデックスを1つ進める
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            // 次の文字
            self.next();
        }
        // 解析中の文字を取得して字句に変換する
        let curr = self.curr()?;
        let token = if Self::is_number(curr) {
            // 数字の場合
            let mut number = vec![*curr];
            // 次に解析する文字が数字であった場合・・・
            while self.peek().is_some() && Self::is_number(self.peek().unwrap()) {
                // 次の文字へインデックスを一つ進める
                self.next();
                // 要素をベクタへ追加
                number.push(*self.curr().unwrap());
            }
            String::from_iter(number)
                // `f64`型へパース
                .parse::<f64>()
                // 結果を`Result<T, E>`から`Option<T>`へ変換
                .ok()
                // `Option<T>`にwrapされた値を返す
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
    // 解析中の文字
    fn curr(&mut self) -> Option<&char> {
        // `get`で解析中の要素への参照を返す
        self.input.get(self.position)
    }
    // 次に解析する文字
    fn peek(&mut self) -> Option<&char> {
        // `get`で次に解析する要素への参照を返す
        self.input.get(self.position + 1)
    }
    // 文字が数字であるかどうか
    // `is_ascii_digit`で、値がASCIIの10進数であるかチェック
    fn is_number(c: &char) -> bool {
        // `||`は論理OR
        c.is_ascii_digit() || c == &'.'
    }
}

#[derive(Debug)]
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

struct Parser {
    // 字句解析器
    lexer: Lexer,
    // 現在解析中の字句
    curr: Option<Token>,
    // 次に解析する字句
    peek: Option<Token>,
}

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
    // 初期化
    fn new(mut lexer: Lexer) -> Parser {
        let curr = lexer.token();
        let peek = lexer.token();
        Parser { 
            lexer, 
            curr, 
            peek 
        }
    }

    // 次の字句を解析対象にする
    fn next(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.token();
    }

    // 構文木の葉の要素の解析
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        // `as_ref`で&Option<T>をOption<&T>へ変換
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

    fn token_precedence(token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::SUM,
            Token::Slash | Token::Asterisk => Precedence::PRODUCT,
            _ => Precedence::LOWEST,
        }
    }

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

fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::PrefixExpr { 
            operator: _,
            right,
        } => -eval(right),
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