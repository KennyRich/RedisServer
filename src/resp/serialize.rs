use concat_string::concat_string;
use std::borrow::Cow;

//In RESP, the first byte determines the data type:
//
// For Simple Strings, the first byte of the reply is "+"
// For Errors, the first byte of the reply is "-"
// For Integers, the first byte of the reply is ":"
// For Bulk Strings, the first byte of the reply is "$"
// For Arrays, the first byte of the reply is "*"

#[derive(PartialEq)]
pub enum InputVariants {
    NumberVariant(i32),
    StringVariant(String),
    ErrorVariant(String),
    StringArrayVariant(Vec<String>),
    Null,
}

pub fn serialize(input: InputVariants) -> Cow<'static, str> {
    match input {
        InputVariants::NumberVariant(number) => {
            Cow::Owned(concat_string!(":", number.to_string(), "\r\n"))
        }
        InputVariants::StringVariant(string) if string.starts_with("+") => {
            Cow::Owned(concat_string!(string, "\r\n"))
        }
        InputVariants::StringVariant(string) => Cow::Owned(concat_string!(
            "$",
            string.len().to_string(),
            "\r\n",
            string,
            "\r\n"
        )),
        InputVariants::ErrorVariant(string) => Cow::Owned(concat_string!(string, "\r\n")),
        InputVariants::StringArrayVariant(string_array) => {
            let serialized_items: Vec<String> = string_array
                .iter()
                .map(|item| serialize(InputVariants::StringVariant(item.clone())).into_owned())
                .collect();
            Cow::Owned(concat_string!(
                "*",
                serialized_items.len().to_string(),
                "\r\n",
                serialized_items.concat().to_string()
            ))
        }
        _ => Cow::Borrowed("$-1\r\n"),
    }
}
