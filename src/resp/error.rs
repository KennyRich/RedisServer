// Using self ensures that you explicitly include all the module along with its content
use std::fmt::{self, Formatter};

#[derive(Debug, PartialEq)]
pub enum ErrorMessages {
    MissingBulkString,
    EmptyInput,
    UnknownInput(String),
    ParseError(String),
    UnexpectedVariant,
}

impl fmt::Display for ErrorMessages {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorMessages::EmptyInput => write!(f, "Input cannot be empty!"),
            ErrorMessages::MissingBulkString => write!(f, "Bulk String cannot be empty or null!"),
            ErrorMessages::UnexpectedVariant => write!(f, "Unexpected variant!"),
            ErrorMessages::UnknownInput(details) => write!(f, "Unknown input {}", details),
            ErrorMessages::ParseError(details) => write!(
                f,
                "String might be containing unparseable length: {}",
                details
            ),
        }
    }
}

impl std::error::Error for ErrorMessages {}
