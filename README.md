# OXL - Oxide Config Language

OXL is a simple, fast config language parser written in Rust.

It tokenizes `.oxl` files, validates syntax, and transpiles them into Rust structs with values.

---

## Features

- Fast single-pass tokenizer
- Lightweight syntax validation
- Simple CFG-style parsing model
- No AST overhead
- Transpiles to Rust structs with values

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
struct window {
    title: String,
    width: i64,
    height: i64,
    fullscreen: bool,
}

let window = window {
    title: "oxide editor".to_string(),
    width: 1920,
    height: 1080,
    fullscreen: false,
};

struct audio {
    volume: f64,
}

let audio = audio {
    volume: 0.8,
};
```

## Installation

```bash
cargo add oxideconf
```