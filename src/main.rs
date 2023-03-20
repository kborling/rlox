
fn main() -> sysexits::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let len = args.len();
    if len != 2 {
        println!("Usage: jlox [script]");
        return sysexits::ExitCode::Usage
    }
    let file_path = &args[1];
    println!("{}", &file_path);

    sysexits::ExitCode::Ok
}
