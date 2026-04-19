# OXL - Oxide Config Language

OXL is a simple, fast config language parser written in Rust.

It tokenizes `.oxl` files, validates syntax, and transpiles them into Rust structs.

---

## Features

- Fast single-pass tokenizer
- Lightweight syntax validation
- Simple CFG-style parsing model
- No AST overhead
- Transpiles to Rust structs

---

## Syntax

Sections are declared with `!` followed by a name. Variables are declared using a name followed by a value separated by whitespace. `;` is used for comments.

```oxl
; oxide config

!window
title "oxide editor"
width 1920
height 1080
fullscreen false

!audio
volume 0.8
```

## Usage

```rust
use oxideconf::compile;

fn main() {
    let input = r#"
!window
title "oxide editor"
width 1920
height 1080
fullscreen false

!audio
volume 0.8
"#;
    
    match oxideconf::compile(input.to_string()) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("{}", e),
    }
}
```

## Example output

```rust
struct Window {
    title: String,
    width: i64,
    height: i64,
    fullscreen: bool,
}

struct Audio {
    volume: f64,
}
```

## Installation

```bash
cargo add oxideconf
```