use crate::lexer::Token;
use crate::error::{Error, ErrorType};

enum State {
    ExpectIdentifier,
    ExpectAssign,
    ExpectValue,
    ExpectNewline,
}

pub fn validate(tokens: &[Token]) -> Result<(), Error> {
    let mut state = State::ExpectIdentifier;

    for token in tokens {
        state = match state {

            State::ExpectIdentifier => {
                match token {
                    Token::Identifier(_) => State::ExpectAssign,
                    Token::Newline => State::ExpectIdentifier, // allow empty lines
                    _ => return Err(unexpected("Identifier")),
                }
            }

            State::ExpectAssign => {
                match token {
                    Token::Assign => State::ExpectValue,
                    _ => return Err(unexpected("ASSIGN")),
                }
            }

            State::ExpectValue => {
                match token {
                    Token::Int(_) | Token::Float(_) => State::ExpectNewline,
                    _ => return Err(unexpected("Int or Float")),
                }
            }

            State::ExpectNewline => {
                match token {
                    Token::Newline => State::ExpectIdentifier,
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