use super::token::Token;
use super::token_type::TokenType;

pub struct Scanner<T> {
    source: String,
    tokens: Vec<Token<T>>,
    start: i32,
    current: i32,
    line: i32
}

impl<T> Scanner<T> {
    fn new(source: String) -> Scanner<T> {
        Scanner {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len() as i32
    }

    fn scan_token(&mut self) {
        println!("to be done tomorrow");
    }

    fn scan_tokens(&mut self) -> &Vec<Token<T>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token { type_token: TokenType::EOF, lexeme: String::from(""), literal: None, line: self.line });
        &self.tokens
    }
}
