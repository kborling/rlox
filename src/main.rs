#![warn(clippy::all, clippy::pedantic)]
mod token;
mod scanner;
use std::{
    fs::File,
    io::{BufRead, ErrorKind, Write},
};

pub use token::Token;
use scanner::Scanner;

fn main() -> sysexits::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();

    if len == 1 {
        run_prompt();
    } else if len == 2 {
        run_file();
    } else {
        println!("Usage: jlox <script>");
        std::process::exit(sysexits::ExitCode::Usage as i32);
    }

    sysexits::ExitCode::Ok
}

/**
 * Run the REPL
 */
fn run_prompt() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        Scanner::default().run(line);
    }
}

/**
 * Run a file
 */
fn run_file() {
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
        Scanner::default().run(line);
    }
}

