use RedisServer::resp::utils::{read_array, read_bulk_string, read_simple_string, split_data};

#[test]
fn should_split_data() {
    assert_eq!(
        split_data("$5\r\nworld\r\n").unwrap(),
        ("$5".to_string(), "world\r\n".to_string())
    );
}

#[test]
fn should_deserialize_bulk_string() {
    let input = "5\r\nworld\r\n";
    assert_eq!(read_bulk_string(input).unwrap().0, "world".to_string())
}

#[test]
fn should_not_deserialize_bulk_string() {
    let input = "-1\r\nworld\r\n";
    assert!(read_bulk_string(input).is_err(),);
}

#[test]
fn should_deserialize_simple_string() {
    let input = "OK\r\n";
    assert_eq!(read_simple_string(input).unwrap().0, "OK".to_string());
}

#[test]
fn should_deserialize_array_of_bulk_string() {
    let input = "2\r\n$4\r\necho\r\n$11\r\nhello world\r\n";
    assert_eq!(read_array(input).unwrap().0, vec!["echo", "hello world"]);
}
