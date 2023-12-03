use std::{borrow::Cow};
use RedisServer::{
    connection_manager::{
        command_handlers::handle_command,
        utils::{throw_err_if_num_args_wrong, serialize_error}
    }
};


#[test]
fn should_return_serialized_pong(){
    let input = Cow::Borrowed("*1\r\n$4\r\nPING\r\n");
    assert_eq!("$4\r\nPong\r\n", handle_command(input))
}

#[test]
fn should_echo_hello_world() {
    let input = Cow::Borrowed("*2\r\n$4\r\nECHO\r\n$11\r\nHELLO WORLD\r\n");
    assert_eq!(
        "$11\r\nHELLO WORLD\r\n",
        handle_command(input)
    );
}

#[test]
fn should_return_error_when_echo_have_too_many_args() {
    let input = Cow::Borrowed("*3\r\n$4\r\nECHO\r\n$4\r\nHEHE\r\n$4\r\nHEHE\r\n");
    assert_eq!(
        throw_err_if_num_args_wrong("echo"),
        handle_command(input)
    );
}

#[test]
fn should_return_error_when_fail_to_deserialize() {
    let input = Cow::Borrowed("*1\r\nSILLY");
    assert_eq!(
        serialize_error("-failed to deserialize"),
        handle_command(input)
    );
}

#[test]
fn should_return_error_when_unknown_command() {
    let input = Cow::Borrowed("*2\r\n$5\r\nECHOO\r\n$2\r\nRT\r\n");
    assert_eq!(
        serialize_error(format!("-unknown command '{}'", "echoo").as_str()),
        handle_command(input)
    );
}
