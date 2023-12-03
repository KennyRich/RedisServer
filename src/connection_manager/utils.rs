use crate::resp::serialize::{serialize, InputVariants};
use concat_string::concat_string;
use std::borrow::Cow;

pub fn throw_err_if_num_args_wrong(variant: &str) -> Cow<'static, str> {
    serialize(InputVariants::StringVariant(concat_string!(
        "-ERR wrong number of argument for",
        variant,
        " command"
    )))
}

pub fn serialize_error(message: &str) -> Cow<'static, str> {
    println!("{}", message);
    serialize(InputVariants::ErrorVariant(message.to_string()))
}
