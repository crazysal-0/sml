enum Token {
    Identifier(String),
    Section(String),
    Newline,
    ASSIGN,

    Int(i64),
    Float(f64),
    StringVal(String),
    Bool(bool),
}

pub fn generate_tokens(source: String) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut index: usize = 0;

    while index < chars.len() {
        let current_character = chars[index];

        if current_character == '\n' {
            tokens.push(Token::Newline);
            index += 1;
        }

        else if current_character == ' ' {
            tokens.push(Token::ASSIGN);
            index += 1;
        }

        else if current_character.is_alphabetic() {
            let mut buffer = String::new();

            while index < chars.len()
                && (chars[index].is_alphanumeric() || chars[index] == '_')
            {
                buffer.push(chars[index]);
                index += 1;
            }

            tokens.push(Token::Identifier(buffer));
        }

        else if current_character.is_numeric() {
            let mut number_string = String::new();

            while index < chars.len() &&
                  (chars[index].is_numeric() || chars[index] == '.') {

                number_string.push(chars[index]);
                index += 1;
            }

            let dot_count = number_string.chars()
                .filter(|&c| c == '.')
                .count();

            if dot_count == 0 {
                tokens.push(Token::Int(number_string.parse().unwrap()));
            }
            else if dot_count == 1 {
                tokens.push(Token::Float(number_string.parse().unwrap()));
            }
            else {
                return Err(Error {
                    error_type: ErrorType::IllegalCharacterError,
                    description: "Too many dots in float".to_string(),
                });
            }
        }

        else {
            return Err(Error {
                error_type: ErrorType::IllegalCharacterError,
                description: format!("Illegal character: {}", current_character),
            });
        }
    }

    Ok(tokens)
}