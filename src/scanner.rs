use std::{
    fs::File,
    io::{ErrorKind, Write, BufRead},
};

use crate::{Token, token::TokenType};

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
        while self.current >= self.source.len() {
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
        self.current += 1;
        let c: char = self.source.chars().nth(self.current).unwrap();

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
            _ => ()
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self.source.chars().skip(self.start).take(self.current).collect();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal: None,
            line: self.line,
        });
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.chars().for_each(|c| print!("{c}"));
            self.had_error = false;
        }
    }

    pub fn run_file(&self) {
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
            line.chars().for_each(|c| print!("{c}"));
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, line_number: &str, message: &str) {
        println!("[line {line}] Error{line_number}: {message}");
        self.had_error = true;
    }
}
