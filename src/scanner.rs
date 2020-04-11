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

    fn advance(&mut self) -> String {
        self.current += 1;
        let source_vec = self.source.chars().nth(self.current - 1).unwrap();
        source_vec.to_string()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type, lexeme: text.to_string(), literal: None, line: self.line });
    }

    fn scan_token(&mut self) {
        let current_char = self.advance();
        match current_char.as_str() {
            "(" => self.add_token(TokenType::LEFT_PAREN),
            ")" => self.add_token(TokenType::RIGHT_PAREN),
            "{" => self.add_token(TokenType::LEFT_BRACE),
            "}" => self.add_token(TokenType::RIGHT_BRACE),
            "," => self.add_token(TokenType::COMMA),
            "." => self.add_token(TokenType::DOT),
            "-" => self.add_token(TokenType::MINUS),
            "+" => self.add_token(TokenType::PLUS),
            ";" => self.add_token(TokenType::SEMICOLON),
            "*" => self.add_token(TokenType::STAR),
            _ => println!("Token Type not recoginized"),
        }
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
