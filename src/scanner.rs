use core::any::Any;

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
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
    }

    fn scan_token(&mut self) {
        let c: char = self.peek();
        self.current += 1;

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
            '!' => match self.match_tokens('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang),
            },
            '=' => match self.match_tokens('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },
            '<' => match self.match_tokens('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less),
            },
            '>' => match self.match_tokens('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater),
            },
            '/' => match self.match_tokens('/') {
                true => {
                    while self.peek() != '\n' && !self.at_end() {
                        self.current += 1;
                    }
                }
                false => self.add_token(TokenType::Slash),
            },
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string_literal(),
            _ => {
                if self.is_digit(c) {
                    self.number_literal();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.error("Unexpected character.")
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current)
            .collect();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
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

    fn string_literal(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }

        if self.at_end() {
            self.error("Unterminated string.");
            return;
        }

        self.current += 1;

        let literal_value = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - 2)
            .collect::<String>();
        self.add_token_literal(TokenType::String, Some(Box::new(literal_value)));
    }

    fn number_literal(&mut self) {
        while self.is_digit(self.peek()) {
            self.current += 1;
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.current += 1;

            while self.is_digit(self.peek()) {
                self.current += 1;
            }
        }
        let literal_value = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current)
            .collect::<String>();
        let literal_value = literal_value.parse::<f64>();
        let literal_value = match literal_value {
            Ok(v) => v,
            Err(_) => {
                self.error("Invalid number.");
                return;
            }
        };
        self.add_token_literal(TokenType::Number, Some(Box::new(literal_value)));
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.current += 1;
        }

        let text = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current)
            .collect::<String>();

        let token_type = Token::parse_keyword(&text);

        match token_type {
            Some(token_type) => self.add_token(token_type),
            None => self.add_token(TokenType::Identifier),
        }
    }

    // fn is_reserved(&self, word: &str) -> bool {}

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        let current_char = self.source.chars().nth(self.current);

        match current_char {
            Some(c) => c,
            None => '\0',
        }
    }

    fn peek_next(&self) -> char {
        let current_char = self.source.chars().nth(self.current + 1);

        match current_char {
            Some(c) => c,
            None => '\0',
        }
    }

    fn error(&mut self, message: &str) {
        self.report(self.line, "", message);
    }

    fn report(&mut self, line: i32, line_number: &str, message: &str) {
        println!("[line {line}] Error {line_number}: {message}");
        self.had_error = true;
    }
}
