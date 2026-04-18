use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    IllegalCharacterError,
    SyntaxError,
}

#[derive(Debug)]
pub struct Error {
    pub error_type: ErrorType,
    pub description: String,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::IllegalCharacterError => write!(f, "IllegalCharacterError"),
            ErrorType::SyntaxError => write!(f, "SyntaxError"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.description)
    }
}