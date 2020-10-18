use super::token::Token;
use super::interpreter::Interpreter;
use super::token_type::TokenType;
use std::str;

// Scanner struct
// source:  a list of bytes containing the source code
// start: number containing the start of the source code
// current: number tracking the current char in the source the program is at
// line: number tracking the line number we're at in the source code
// interpreter: contaings the object interpreter the scanner is isued from 
// (error reporting is on a higher level therefore its necessary)
pub struct Scanner<T> {
    source: Vec<u8>,
    tokens: Vec<Token<T>>,
    start: usize,
    current: usize,
    line: usize,
    interpreter: Interpreter
}

impl<T> Scanner<T> {
    // Method to create a new scanner with default data in fields
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

    // method to advance of one char in the source code returning the current char
    fn advance(&mut self) -> String {
        self.current += 1;
        let current_char = self.source[self.current] as char;
        current_char.to_string()
    }

    // method to change a source of bytes to a string
    fn source_to_string(&self, source: &[u8]) -> String {
        str::from_utf8(&source).unwrap().to_string()
    }

    // method to add a token to the .tokens list
    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token { token_type, lexeme: self.source_to_string(text), literal: None, line: self.line });
    }

    // method to match a token at the current spot the .source field is set
    fn match_token(&mut self, expected_char: &str) -> bool {
        if self.is_at_end() {
            return false
        }
        // self.source[self.current] = current char in the source (in byte)
        // expected_char needs to be converted to bytes due to str
        // its a list of bytes of length 1 if ascii [0] required
        if self.source[self.current] != expected_char.as_bytes()[0] {
            return false
        }
        self.current += 1;
        true
    }

    // method to scan tokens in the source code at the current char
    // add the token to the tokens list if matches are found
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
            "!" => if self.match_token("=") { self.add_token(TokenType::BANG_EQUAL) } else { self.add_token(TokenType::BANG) }
            "=" => if self.match_token("=") { self.add_token(TokenType::EQUAL_EQUAL) } else { self.add_token(TokenType::EQUAL) }
            "<" => if self.match_token("=") { self.add_token(TokenType::LESS_EQUAL) } else { self.add_token(TokenType::LESS) }
            ">" => if self.match_token("=") { self.add_token(TokenType::GREATER_EQUAL) } else { self.add_token(TokenType::GREATER) }
            _ => self.interpreter.error(self.line, String::from("Unexpected Character")),
        }
    }

    // method to detected if the program has reached the end of the source code
    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len()
    }

    // method to begin the scanning of the tokens stop the program has reached the end of the source code
    // returns the full list of the scanned tokens
    fn scan_tokens(&mut self) -> &Vec<Token<T>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token { token_type: TokenType::EOF, lexeme: String::from(""), literal: None, line: self.line });
        &self.tokens
    }
}
