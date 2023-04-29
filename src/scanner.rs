use crate::{token::TokenType, Token};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
    had_error: bool,
}

impl Scanner {
    pub fn default() -> Self {
        Self {
            source: "".to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    pub fn run(&mut self, source: String) {
        self.source = source;
        self.scan_tokens();

        println!("{:?}", self.tokens);
    }

    fn scan_tokens(&mut self) {
        while self.current < self.source.len() - 1 {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                match self.match_tokens('=') {
                    true => self.add_token(TokenType::BangEqual),
                    false => self.add_token(TokenType::Bang),
                }
            },
            '=' => {
                match self.match_tokens('=') {
                    true => self.add_token(TokenType::EqualEqual),
                    false => self.add_token(TokenType::Equal),
                }
            },
            '<' => {
                match self.match_tokens('=') {
                    true => self.add_token(TokenType::LessEqual),
                    false => self.add_token(TokenType::Less),
                }
            },
            '>' => {
                match self.match_tokens('=') {
                    true => self.add_token(TokenType::GreaterEqual),
                    false => self.add_token(TokenType::Greater),
                }
            },
            '/' => {
                match self.match_tokens('/') {
                    true => {
                        while self.peek() != '\n' && !self.at_end() {
                            self.advance();
                        }
                    },
                    false => self.add_token(TokenType::Slash),
                }
            },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            _ => {
                self.error(self.line, "Unexpected character.");
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current)
            .collect();
        
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: None,
            line: self.line,
        });
    }

    fn match_tokens(&mut self, expected: char) -> bool {
        
        if self.at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }
        
        self.current += 1;
        true
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c: char = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap()
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, line_number: &str, message: &str) {
        println!("[line {line}] Error{line_number}: {message}");
        self.had_error = true;
    }
}
