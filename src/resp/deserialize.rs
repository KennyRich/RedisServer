use super::error::ErrorMessages;
use super::utils::{read_array, read_bulk_string, read_simple_string};

#[derive(Debug, PartialEq)]
pub enum RESPResponse {
    TupleVariant(String, String),
    VecVariant(Vec<String>, String),
}

pub fn deserialize(serialized_input: &str) -> Result<RESPResponse, ErrorMessages> {
    if serialized_input == "$-1\r\n" {
        return Err(ErrorMessages::MissingBulkString);
    }
    if serialized_input.is_empty() {
        return Err(ErrorMessages::EmptyInput);
    }
    let (first_char, serialized_input) = match serialized_input.chars().next() {
        Some(ch) => (ch, &serialized_input[1..]),
        None => return Err(ErrorMessages::EmptyInput),
    };

    match first_char {
        '+' | '-' => {
            let (head, tail) = read_simple_string(serialized_input)?;
            Ok(RESPResponse::TupleVariant(head, tail))
        }
        '$' => read_bulk_string(serialized_input)
            .map(|(head, tail)| RESPResponse::TupleVariant(head, tail)),

        '*' => {
            read_array(serialized_input).map(|(head, tail)| RESPResponse::VecVariant(head, tail))
        }
        details => Err(ErrorMessages::UnknownInput(details.to_string())),
    }
}
