use super::{
    deserialize::{deserialize, RESPResponse},
    error::ErrorMessages,
};

pub static END_OF_LINE: &str = "\r\n";

pub fn split_data(serialized_input: &str) -> Result<(String, String), &'static str> {
    // '_ is a lifetime parameter defined by Rust
    let (head, tail) = serialized_input
        .split_once("\r\n")
        .ok_or("No head element found")?;

    Ok((head.to_string(), tail.to_string()))
}

pub fn read_bulk_string(serialized_input: &str) -> Result<(String, String), ErrorMessages> {
    match split_data(serialized_input) {
        Ok((string_length, value)) => {
            let parsed_string_length = string_length
                .parse::<usize>()
                .map_err(|err| ErrorMessages::ParseError(err.to_string()))?;
            let bulk_string_value: String = value.chars().take(parsed_string_length).collect();
            let remaining_tail: String = value.chars().skip(parsed_string_length + 2).collect();
            Ok((bulk_string_value, remaining_tail))
        }
        Err(err) => Err(ErrorMessages::ParseError(err.to_string())),
    }
}

pub fn read_simple_string(serialized_input: &str) -> Result<(String, String), ErrorMessages> {
    match split_data(serialized_input) {
        Ok(ok_val) => Ok(ok_val),
        Err(err) => Err(ErrorMessages::ParseError(err.to_string())),
    }
}

pub fn read_array(data: &str) -> Result<(Vec<String>, String), ErrorMessages> {
    match split_data(data) {
        Ok((array_length, mut remaining_data)) => {
            let count = array_length
                .parse::<usize>()
                .map_err(|err| ErrorMessages::UnknownInput(err.to_string()))?;
            let mut items: Vec<String> = Vec::new();

            for _ in 0..count {
                let parsed_item = deserialize(&remaining_data)?;
                match parsed_item {
                    RESPResponse::TupleVariant(head, tail) => {
                        remaining_data = tail;
                        items.push(head);
                    }
                    RESPResponse::VecVariant(_, _) => {
                        return Err(ErrorMessages::UnexpectedVariant);
                    }
                }
            }
            Ok((items, remaining_data))
        }
        Err(err) => Err(ErrorMessages::ParseError(err.to_string())),
    }
}
