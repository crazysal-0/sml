use std::{env, process::exit};
use std::fs;

mod error;
mod lexer;
mod validator;
mod codegen;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() != 2 {
        println!("Incorrect usage!");
        println!("Expected: sml <file.sml>");
        exit(1);
    }

    let source = match fs::read_to_string(&arguments[1]) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            exit(1);
        }
    };

    let tokens = match lexer::generate_tokens(source) {
        Ok(t) => t,
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    };

    println!("{:?}", tokens);

    if let Err(e) = validator::validate(&tokens) {
        println!("{}", e);
        exit(1);
    }

    let output = codegen::generate_code(&tokens);

    println!("\n--- GENERATED CODE ---\n");
    println!("{}", output);
}