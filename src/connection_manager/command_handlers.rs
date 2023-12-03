use::std::borrow::Cow;
use super::{
    commands::{handle_echo, handle_ping, ignore_command},
    utils::serialize_error
};
use crate::{
    resp::{deserialize::{RESPResponse, deserialize} },
    // database::db::Cache
};

pub fn handle_command(human_readable: Cow<'_, str>) -> Cow<'static, str>{
    let command = deserialize(&human_readable);
    match command{
        Ok(deserialized_command) => match deserialized_command{
            RESPResponse::VecVariant(commands, _) => {
                if let Some(command) = commands.first().map(|s| s.to_lowercase()){
                    let args = &commands[1..];
                    match command.as_str(){
                        "command" =>ignore_command(),
                        "ping" => handle_ping(),
                        "echo" => handle_echo(args),
                        unknown_command => {
                            let message = "-unknown command '".to_owned() + unknown_command + "'";
                            serialize_error(message.as_str())
                        }
                    }
                } else {
                    serialize_error("-commands array is empty")
                }
            }
            _ =>serialize_error("-unsupported RESP type")
        }
        Err(err) =>{
            print!("{}", err);
            serialize_error("-failed to deserialize")
        }
    }
}