use std::{
    fs::File,
    io::{BufRead, ErrorKind, Write},
};

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
            source: String::new(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    fn scan_tokens(&mut self) {
        while self.current < self.source.len() {
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
        let c: char = self.source.chars().nth(self.current).unwrap();
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
            _ => self.error(self.line, "Unexpected character.")
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
        
        if self.current >= self.source.len() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        
        self.current += 1;
        true
    }

    fn run(&mut self, source: String) {
        self.source = source;
        self.scan_tokens();

        println!("{:?}", self.tokens);
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();

            self.run(line);
        }
    }

    pub fn run_file(&mut self) {
        let args: Vec<String> = std::env::args().collect();
        let path = &args[1];
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => {
                if error.kind() == ErrorKind::NotFound {
                    println!("File not found: {path}");
                    std::process::exit(sysexits::ExitCode::NoInput as i32);
                } else {
                    println!("Error opening file: {error}");
                    std::process::exit(sysexits::ExitCode::Software as i32);
                }
            }
        };
        let reader = std::io::BufReader::new(&file);
        for line in reader.lines().flatten() {
            self.run(line);
        }
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, line_number: &str, message: &str) {
        println!("[line {line}] Error{line_number}: {message}");
        self.had_error = true;
    }
}
