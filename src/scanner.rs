use super::token::Token;
use super::interpreter::Interpreter;
use super::token_type::TokenType;
use std::str;

pub struct Scanner<T> {
    source: Vec<u8>,
    tokens: Vec<Token<T>>,
    start: usize,
    current: usize,
    line: usize,
    interpreter: Interpreter
}

impl<T> Scanner<T> {
    fn new(source: String, interpreter: Interpreter) -> Scanner<T> {
        Scanner {
            source: source.into_bytes(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
            interpreter
        }
    }

    fn advance(&mut self) -> String {
        self.current += 1;
        let current_char = self.source[self.current - 1];
        current_char.to_string()
    }

    fn source_to_string(&self, source: &[u8]) -> String {
        str::from_utf8(&source).unwrap().to_string()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type, lexeme: self.source_to_string(text), literal: None, line: self.line });
    }

    fn match_token(&mut self, expected_char: &str) -> bool {
        if self.is_at_end() {
            return false
        }
        if self.source[self.current - 1] != expected_char.as_bytes()[0] {
            return false
        }
        self.current += 1;
        return true
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
            _ => self.interpreter.error(self.line, String::from("Unexpected Character")),
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
