use super::token::Token;
use super::token_type::TokenType;

pub struct Scanner<T> {
    source: String,
    tokens: Vec<Token<T>>,
    start: usize,
    current: usize,
    line: usize
}

impl<T> Scanner<T> {
    fn new(source: String) -> Scanner<T> {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }


    fn scan_token(&mut self) {
        println!("to be done tomorrow");
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let source_vec = self.source.chars().nth(self.current - 1).unwrap();
        source_vec
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type, lexeme: text.to_string(), literal: None, line: self.line });
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len()
    }

    fn scan_tokens(&mut self) -> &Vec<Token<T>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token { token_type: TokenType::EOF, lexeme: String::from(""), literal: None, line: self.line });
        &self.tokens
    }
}
