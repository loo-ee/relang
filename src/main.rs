use std::{env, fs, io::{Write}, process};
use std::sync::Mutex;

mod token_type;
mod token;
mod scanner;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref HAD_ERROR: Mutex<bool> = Mutex::new(false);
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let _ = run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> std::io::Result<()> {
    let had_error = {
        let lock = HAD_ERROR.lock().unwrap();
        *lock
    };

    let contents = fs::read_to_string(&path)?;
    run(contents);
    
    if had_error {
        println!("test");
        process::exit(65);
    }
    Ok(())
}

fn run_prompt() {
    let mut had_error = HAD_ERROR.lock().unwrap();

    loop {
        std::io::stdout().flush().unwrap();
        print!("> "); 
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Error reading line"); 
        if line.is_empty() { break };
        run(line);
        *had_error = false;
    }
}

fn run(contents: String) {
    let mut scanner = scanner::Scanner::new(contents);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn error(line: usize, message: String) {
    report(line, "".to_string(), message);
}

fn report(line: usize, loc: String, message: String) {
    let mut had_error = HAD_ERROR.lock().unwrap();

    eprintln!("[line {} ] Error {}: {}", line, loc, message);
    *had_error = true;
}
