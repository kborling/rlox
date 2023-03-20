use std::{
    fs::File,
    io::{BufRead, ErrorKind},
};

pub struct Scanner {
    file: File,
}

impl Scanner {
    pub fn default() -> Self {
        let args: Vec<String> = std::env::args().collect();
        if args.len() != 2 {
            println!("Usage: jlox <script>");
            std::process::exit(sysexits::ExitCode::Usage as i32);
        }
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
        Self { file }
    }

    pub fn run(&self) {
        // println!("File: {:?}", self.file);
        let reader = std::io::BufReader::new(&self.file);
        for line in reader.lines().flatten() {
            line.chars().for_each(|c| print!("{c}"));
        }
    }
}
