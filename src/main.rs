pub mod parser;
pub mod scanner;
pub mod token;

use std::env;
use std::io::{self, Write};

use parser::visitor::{GraphGenerator, PrettyPrinter, Visitor};
use parser::Parser;
use scanner::Scanner;

use clap::{arg, Command};

fn get_args_parser() -> Command {
    Command::new("jlox")
    .version("1.0")
    .about("jlox repl and interpreter")
    .args(&[
        arg!(-s --syntaxtree "generate svg with the syntaxt tree"),
        arg!([script] "an optional script file to interpret. If this is missing a repl will be launched instead"),
    ])
}

fn main() {
    let arg_matches = get_args_parser().get_matches();
    let syntax_tree = arg_matches.get_flag("syntaxtree");
    match arg_matches.get_one::<String>("script") {
        Some(script) => run_file(script, syntax_tree),
        None => run_prompt(syntax_tree),
    }
}

fn run_prompt(syntax_tree: bool) {
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
                if syntax_tree {
                    let graph_generator = GraphGenerator::new();
                    match graph_generator.generate_tree(&expr_tree) {
                        Err(error) => println!("Error trying to generate the graph: {}", error),
                        _ => {}
                    }
                }
            }
            Err(err) => println!("Cannot read command line input: {}", err),
        }
    }
}

fn run_file(path: &String, syntax_tree: bool) {
    // Open the file in read-only mode.
    match std::fs::read_to_string(path) {
        Ok(script) => {
            let mut scanner = Scanner::new(&script);
            let tokens = scanner.scan_tokens();

            let mut parser = Parser::new(tokens);
            let expr_tree = parser.parse();
            if syntax_tree {
                let graph_generator = GraphGenerator::new();
                match graph_generator.generate_tree(&expr_tree) {
                    Err(error) => println!("Error trying to generate the graph: {}", error),
                    _ => {}
                }
            }
        }
        Err(err) => {
            println!("could not read the script: {}", err);
            std::process::exit(64)
        }
    }
}
