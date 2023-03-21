use std::{
    fs::File,
    io::{ErrorKind, Write, BufRead},
};


#[derive(Default)]
pub struct Scanner {
    had_error: bool,
}

impl Scanner {
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
        self.had_error = true
    }
}
