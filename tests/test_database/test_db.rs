use RedisServer::database::db::Cache;
use std::time::Duration;
use std::thread;


#[test]
fn test_should_initialise_get_and_set() {
    let cache = Cache::new();
    cache.set(String::from("name"), String::from("Harry Potter")).unwrap();
    assert_eq!("Harry Potter", cache.get("name").unwrap().unwrap())
}

#[test]
fn test_should_set_and_get_expiration() {
    let cache = Cache::new();
    cache.set_with_ttl(String::from("movie"), String::from("Home Alone"), Duration::from_secs(1)).unwrap();
    thread::sleep(Duration::from_secs(2));
    assert!(cache.get("movie").unwrap().is_none());
}