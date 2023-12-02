use RedisServer::resp::serialize::{serialize, InputVariants};

#[test]
fn should_serialize_null_to_bulk() {
    assert_eq!(serialize(InputVariants::Null), "$-1\r\n".to_string());
}

#[test]
fn should_serialize_bulk_string() {
    assert_eq!(
        serialize(InputVariants::StringVariant("PING".to_string())),
        "$4\r\nPING\r\n"
    );
}

#[test]
fn should_serialize_array_of_bulk_string() {
    assert_eq!(
        serialize(InputVariants::StringArrayVariant(vec!(
            "echo".to_string(),
            "hello world".to_string()
        ))),
        "*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n"
    )
}

#[test]
fn should_serialize_simple_string() {
    assert_eq!(
        serialize(InputVariants::StringVariant("+PONG".to_string())),
        "+PONG\r\n".to_string()
    )
}

#[test]
fn should_serialize_integer() {
    assert_eq!(
        serialize(InputVariants::NumberVariant(1)),
        ":1\r\n".to_string()
    )
}
