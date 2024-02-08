use crate::error;
use crate::token::Token;
use crate::token_type::TokenType;

use std::sync::Mutex;

lazy_static! {
    static ref START: Mutex<usize> = Mutex::new(0);
    static ref CURRENT: Mutex<usize> = Mutex::new(0);
    static ref LINE: Mutex<usize> = Mutex::new(1);
}


pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: String,) -> Scanner {
        Scanner {
            source, tokens: Vec::new(), start: 0, current: 0, line: 1
        }
    }
    
    pub fn scan_tokens(&mut self) -> &Vec<Token> {

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
                            TokenType::EOF, 
                            "".to_string(), 
                            "".to_string(),
                            self.line));
                        
        &self.tokens
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token_type: TokenType;

                if self.match_lex('=') { token_type = TokenType::BangEqual }
                else { token_type = TokenType::Bang }

                self.add_token(token_type, None);
            },
            '=' => {
                let token_type: TokenType;

                if self.match_lex('=') { token_type = TokenType::EqualEqual }
                else { token_type = TokenType::Equal }

                self.add_token(token_type, None);
            },
            '<' => {
                let token_type: TokenType;

                if self.match_lex('=') { token_type = TokenType::LessEqual}
                else { token_type = TokenType::Less }

                self.add_token(token_type, None);
            },
            '>' => {
                let token_type: TokenType;

                if self.match_lex('=') { token_type = TokenType::GreaterEqual}
                else { token_type = TokenType::Greater }

                self.add_token(token_type, None);
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            _ => {
                error(self.line, "Unexpected character".to_string());
            }
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1; 
        self.source.chars().nth(self.current -1).expect("Failed to read str")
    }

    pub fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source.get(self.start..self.current).unwrap();

        match literal {
            Some(content) => {
                self.tokens.push(Token::new(token_type, text.to_string(), content, self.line));
            }
            None => self.tokens.push(Token::new(token_type, text.to_string(), "".to_string(), self.line))
        }
    }

    pub fn match_lex(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }
}
