use rust_sessions::cache::Data;

#[test]

fn test_new_cache() {
    let new_cache: Data<i32, &str> = Data::new();
    assert_eq!(new_cache.cache.len(), 0);
}

#[test]

fn test_cache_add() {
    let mut new_cache: Data<i32, &str> = Data::new();
    new_cache.add(1, "oyin");
    assert_eq!(new_cache.cache.len(), 1);
    let retrieved_cache = new_cache.get(1);
    assert_eq!(retrieved_cache, Some(&"oyin"));
}

#[test]

fn test_cache_delete() {
    let mut new_cache: Data<i32, &str> = Data::new();

    assert_eq!(new_cache.cache.len(), 0);

    new_cache.add(1, "oyin");
    assert_eq!(new_cache.cache.len(), 1);
    let retrieved_cache = new_cache.get(1);
    assert_eq!(retrieved_cache, Some(&"oyin"));

    let deleted_cache = new_cache.delete(1);
    assert_eq!(deleted_cache, Ok("oyin"));
}

#[test]

fn test_cache_invalid_delete() {
    let mut new_cache: Data<i32, &str> = Data::new();

    let deleted_cache = new_cache.delete(1);
    assert_eq!(deleted_cache, Err("Data not found"));
}

#[test]

fn test_update_cache() {
    let mut new_cache: Data<i32, &str> = Data::new();
    new_cache.add(1, "oyin");
    assert_eq!(new_cache.cache.len(), 1);
    let retrieved_cache = new_cache.get(1);
    assert_eq!(retrieved_cache, Some(&"oyin"));

    assert_eq!(new_cache.update(1, "oyin"), Ok("Successfully updated"));
}

#[test]

fn test_cache_invalid_update() {
    let mut new_cache: Data<i32, &str> = Data::new();

    let updated_cache = new_cache.update(3, "john");
    println!("{:?}", updated_cache);
    assert_eq!(updated_cache, Err("key you passed doesnt exist"));
}

#[test]

fn test_cache_invalid_get() {
    let mut new_cache: Data<i32, &str> = Data::new();
    let invalid_get = new_cache.get(1);

    assert_eq!(invalid_get, None);
    new_cache.add(5, "shola");
    let valid_get = new_cache.get(5);
    assert_eq!(valid_get, Some(&"shola"));

    let invalid_get = new_cache.get(1);
    assert_eq!(invalid_get, None);
}
