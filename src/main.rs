use std::env;
use std::path::Path;
use std::fs;
use std::io;
use std::io::Write;

fn run_file(filename: &String) {
    let path = Path::new(&filename);
    let content = fs::read_to_string(path)
        .expect("Error reading the file");
    run(content);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        writeln!(&mut stdout, "> ").expect("Failed to get stdout");
        stdout.flush().expect("Failed to flush the stdout content");

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        if input == "exit\n" {
            break;
        }
        run(input);
    }
}

fn run(tokens: String) {
    for token in tokens.chars() {
        println!("{}", token);
    }
}

fn error(line: i32, message: String) {
    report(line, String::from(""), message);
}

fn report(line: i32, location: String, message: String) {
    println!("line: {} Error {}: {}", line, location, message);
    let had_error = true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage panther [file]");
    }
    else if args.len() == 2 {
        run_file(&args[1]);
    }
    else {
        run_prompt();
    }
}

