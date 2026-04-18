use crate::lexer::Token;

pub fn generate_code(tokens: &[Token]) -> String {
    let mut output = String::new();

    output.push_str("struct A {\n");

    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {

            Token::Identifier(name) => {
                if let Some(value_token) = tokens.get(i + 2) { 
                    // skip ASSIGN in between

                    let ty = match value_token {
                        Token::Int(_) => "i64",
                        Token::Float(_) => "f64",
                        Token::StringVal(_) => "String",
                        Token::Bool(_) => "bool",
                        _ => "unknown",
                    };

                    output.push_str(&format!("    {}: {},\n", name, ty));
                }
            }

            _ => {}
        }

        i += 1;
    }

    output.push_str("}\n");

    output
}