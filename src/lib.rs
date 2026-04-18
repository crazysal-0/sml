pub mod lexer;
pub mod validator;
pub mod codegen;
pub mod error;

pub use error::{Error, ErrorType};
pub use lexer::Token;

/// Main SML pipeline entrypoint
pub fn compile(input: String) -> Result<String, Error> {
    let tokens = lexer::generate_tokens(input)?;
    validator::validate(&tokens)?;
    let output = codegen::generate_code(&tokens);
    Ok(output)
}