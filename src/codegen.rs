use crate::lexer::Token;

pub fn generate_code(tokens: &[Token]) -> String {
    let mut output = String::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {

            Token::Section(name) => {
                let mut struct_fields = String::new();
                let mut instance_fields = String::new();

                i += 1;

                // collect all fields for this section
                while i < tokens.len() {
                    match &tokens[i] {
                        Token::Section(_) => break,
                        Token::Identifier(field_name) => {
                            if let Some(value_token) = tokens.get(i + 2) {
                                let (ty, val) = match value_token {
                                    Token::Int(n)       => ("i64".to_string(), format!("{}", n)),
                                    Token::Float(f)     => ("f64".to_string(), format!("{}", f)),
                                    Token::StringVal(s) => ("String".to_string(), format!("\"{}\".to_string()", s)),
                                    Token::Bool(b)      => ("bool".to_string(), format!("{}", b)),
                                    _                   => ("unknown".to_string(), "unknown".to_string()),
                                };

                                struct_fields.push_str(&format!("    {}: {},\n", field_name, ty));
                                instance_fields.push_str(&format!("    {}: {},\n", field_name, val));
                                i += 3;
                            }
                        }
                        _ => { i += 1; }
                    }
                }

                // output struct definition
                output.push_str(&format!("struct {} {{\n", name));
                output.push_str(&struct_fields);
                output.push_str("}\n\n");

                // output instance
                output.push_str(&format!("let {} = {} {{\n", name, name));
                output.push_str(&instance_fields);
                output.push_str("};\n\n");

                continue;
            }

            _ => {}
        }

        i += 1;
    }

    output
}