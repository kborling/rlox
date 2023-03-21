#![warn(clippy::all, clippy::pedantic)]
mod token;
mod scanner;
pub use token::Token;
use scanner::Scanner;

fn main() -> sysexits::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();

    if len == 1 {
        Scanner::default().run_prompt();
    } else if len == 2 {
        Scanner::default().run_file();
    } else {
        println!("Usage: jlox <script>");
        std::process::exit(sysexits::ExitCode::Usage as i32);
    }

    sysexits::ExitCode::Ok
}
