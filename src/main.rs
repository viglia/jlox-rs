pub mod parser;
pub mod scanner;
pub mod token;

use std::env;
use std::io::{self, Write};

use parser::visitor::{GraphGenerator, PrettyPrinter, Visitor};
use parser::Parser;
use scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: jlox-rs [script]");
        std::process::exit(64)
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt()
    }
}

fn run_prompt() {
    let mut input: String = String::new();
    let stdin = std::io::stdin();
    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                println!("Reached EOF");
                break;
            }
            Ok(_) => {
                let mut scanner = Scanner::new(input.as_str());
                let tokens = scanner.scan_tokens();

                let mut parser = Parser::new(tokens);
                let expr_tree = parser.parse();
            }
            Err(err) => println!("Cannot read command line input: {}", err),
        }
    }
}

fn run_file(path: &String) {
    // Open the file in read-only mode.
    match std::fs::read_to_string(path) {
        Ok(script) => {
            let mut scanner = Scanner::new(&script);
            let tokens = scanner.scan_tokens();
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(err) => {
            println!("could not read the script: {}", err);
            std::process::exit(64)
        }
    }
}
