use std::fmt;

enum ErrorType {
    IllegalCharacterError,
    SyntaxError,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::IllegalCharacterError => write!(f, "IllegalCharacterError"),
            ErrorType::SyntaxError => write!(f, "SyntaxError"),
        }
    }
}

struct Error {
    error_type: ErrorType,
    description: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.error_type, self.description)
    }
}