use super::utils::throw_err_if_num_args_wrong;
use crate::resp::serialize::{serialize, InputVariants};
use std::borrow::Cow;

pub fn handle_echo(args: &[String]) -> Cow<'static, str> {
    match args.len() {
        1 => serialize(InputVariants::StringVariant(args[0].clone())),
        _ => throw_err_if_num_args_wrong("echo"),
    }
}

pub fn handle_ping() -> Cow<'static, str> {
    serialize(InputVariants::StringVariant("Pong".to_string()))
}

pub fn ignore_command() -> Cow<'static, str> {
    serialize(InputVariants::Null)
}
