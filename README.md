# Sam Markup Language (SML)

SML is a simple, fast markup/config language parser written in Rust.

It tokenizes `.sml` files, validates syntax, and currently transpiles them into Rust-like structures.

---

## Features

- Fast single-pass tokenizer
- Lightweight syntax validation
- Simple CFG-style parsing model
- No AST overhead

---

## Syntax

Variables are declared using a name followed by a value separated by whitespace.

```sml
sprite_size 32
speed 12.2
```

## Usage

```rust
use sml::compile;

fn main() {
    let input = "sprite_size 32\nspeed 12.2";
    
    match sml::compile(input) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("{}", e),
    }
}
```

## Example output:

```rust
struct A {
    sprite_size: i64,
    speed: f64,
}
```

## Installation

```bash
cargo add sml
```