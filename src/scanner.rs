use crate::error;
use crate::token::{Token, LiteralType};
use crate::token_type::TokenType;

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
                            None,
                            self.line));
                        
        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
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

            // Whitespaces
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,

            // String
            '"' => self.string(),

            // Default
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else {
                    error(self.line, "Unexpected character".to_string());
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1; 
        self.source.chars().nth(self.current -1).expect("Failed to read str")
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LiteralType>) {
        let text = self.source.get(self.start..self.current).unwrap();

        match literal {
            Some(content) => {
                self.tokens.push(Token::new(token_type, text.to_string(), Some(content), self.line));
            }
            None => self.tokens.push(Token::new(token_type, text.to_string(), None, self.line))
        }
    }

    fn match_lex(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1 }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated String.".to_string());
            return;
        }

        self.advance();

        let value = self.source.get(self.start + 1..self.current -1).unwrap();
        self.add_token(TokenType::String, Some(LiteralType::Str(value.into())));
    }

    fn is_digit(&self, c: char) -> bool {
        c  >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) { self.advance(); }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
        
            while self.is_digit(self.peek()) { self.advance(); }
        }

        let substr = self.source.get(self.start..self.current).unwrap();
        self.add_token(TokenType::Number, Some(LiteralType::Num(substr.parse::<f64>().unwrap())));
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0' }
        self.source.chars().nth(self.current + 1).unwrap()
    }
}
