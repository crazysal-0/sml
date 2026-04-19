# OXL - Oxide Config Language

OXL is a simple, fast config language parser written in Rust.

It tokenizes `.oxl` files, validates syntax, and transpiles them into Rust or C structs.

---

## Features

- Fast single-pass tokenizer
- Lightweight syntax validation
- Simple CFG-style parsing model
- No AST overhead
- Transpiles to Rust

---

## Syntax

Variables are declared using a name followed by a value separated by whitespace.

```oxl
width 1920
height 1080
```

## Usage

```rust
use oxidelconf::compile;

fn main() {
    let input = "
        width 1920\n
        height 1080
    ";
    
    match oxidelconf::compile(input.to_string()) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("{}", e),
    }
}
```

## Example output

```rust
struct Config {
    width: i64,
    height: i64,
}
```

## Installation

```bash
cargo add oxidelconf
```
