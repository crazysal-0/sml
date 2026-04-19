use crate::lexer::Token;

pub fn generate_code(tokens: &[Token]) -> String {
    let mut output = String::new();
    let mut current_section = "Config".to_string(); // default if no section
    let mut in_struct = false;
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {

            Token::Section(name) => {
                // close previous struct if open
                if in_struct {
                    output.push_str("}\n\n");
                }

                // capitalize first letter for Rust struct name
                let struct_name = capitalize(name);
                output.push_str(&format!("struct {} {{\n", struct_name));
                current_section = struct_name;
                in_struct = true;
            }

            Token::Identifier(name) => {
                if let Some(value_token) = tokens.get(i + 2) {
                    // skip Assign token in between
                    let ty = match value_token {
                        Token::Int(_)       => "i64",
                        Token::Float(_)     => "f64",
                        Token::StringVal(_) => "String",
                        Token::Bool(_)      => "bool",
                        _                   => "unknown",
                    };

                    output.push_str(&format!("    {}: {},\n", name, ty));
                }
            }

            _ => {}
        }

        i += 1;
    }

    // close last struct
    if in_struct {
        output.push_str("}\n");
    }

    output
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None    => String::new(),
        Some(f) => f.to_uppercase().to_string() + c.as_str(),
    }
}