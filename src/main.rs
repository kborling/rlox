#![warn(clippy::all, clippy::pedantic)]
mod scanner;
use scanner::Scanner;

fn main() -> sysexits::ExitCode {
    Scanner::default().run();
    sysexits::ExitCode::Ok
}
