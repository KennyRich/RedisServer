use RedisServer::resp::deserialize::{deserialize, RESPResponse};
use RedisServer::resp::error::ErrorMessages;

#[test]
fn test_deserialize_array_of_bulk_string() {
    assert_eq!(
        deserialize("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n").unwrap(),
        RESPResponse::VecVariant(
            vec!["echo".to_string(), "hello world".to_string()],
            String::new()
        )
    )
}

#[test]
fn test_deserialize_simple_string() {
    assert_eq!(
        deserialize("+PONG\r\n").unwrap(),
        RESPResponse::TupleVariant(
            "PONG".to_string(),
            "".to_string()
        )
    )
}


#[test]
fn test_deserialize_bulk_string() {
    assert_eq!(
        deserialize("$4\r\nPING\r\n").unwrap(),
        RESPResponse::TupleVariant(
            "PING".to_string(),
            "".to_string()
        )
    )
}

#[test]
fn test_should_return_error_when_empty_bulk_passed(){
    assert_eq!(
        deserialize("$-1\r\n").unwrap_err(),
        ErrorMessages::MissingBulkString
    )
}
