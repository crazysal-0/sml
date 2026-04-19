use crate::lexer::Token;
use crate::error::{Error, ErrorType};

enum State {
    ExpectIdentifierOrSection,
    ExpectAssign,
    ExpectValue,
    ExpectNewline,
}

pub fn validate(tokens: &[Token]) -> Result<(), Error> {
    let mut state = State::ExpectIdentifierOrSection;

    for token in tokens {
        state = match state {

            State::ExpectIdentifierOrSection => {
                match token {
                    Token::Identifier(_) => State::ExpectAssign,
                    Token::Section(_)    => State::ExpectNewline, // !section then newline
                    Token::Newline       => State::ExpectIdentifierOrSection,
                    _ => return Err(unexpected("Identifier or Section")),
                }
            }

            State::ExpectAssign => {
                match token {
                    Token::Assign => State::ExpectValue,
                    _ => return Err(unexpected("Assign")),
                }
            }

            State::ExpectValue => {
                match token {
                    Token::Int(_)       |
                    Token::Float(_)     |
                    Token::StringVal(_) |
                    Token::Bool(_)      => State::ExpectNewline,
                    _ => return Err(unexpected("a value")),
                }
            }

            State::ExpectNewline => {
                match token {
                    Token::Newline => State::ExpectIdentifierOrSection,
                    _ => return Err(unexpected("Newline")),
                }
            }
        }
    }

    Ok(())
}

fn unexpected(expected: &str) -> Error {
    Error {
        error_type: ErrorType::SyntaxError,
        description: format!("unexpected token, expected {}", expected),
    }
}