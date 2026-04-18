use std::{env, fs, process::exit};

use sml::compile; // your lib.rs entrypoint

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: sml <file.sml>");
        exit(1);
    }

    let source = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("Error reading file: {e}");
        exit(1);
    });

    match compile(source) {
        Ok(output) => {
            println!("{output}");
        }
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}