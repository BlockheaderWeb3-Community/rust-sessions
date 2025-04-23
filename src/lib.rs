pub mod cache {
    use std::hash::Hash;
    use std::io::{self, Write};
    use std::{collections::HashMap, str};

    /// This is a struct `Data` with generic types K & V
    /// this struct stores a cache field which has a hash map of
    /// Key of generic type K and value of generic type V
    pub struct Data<K, V> {
        pub cache: HashMap<K, V>,
    }

    // implementation for the struct Data, this is taking a generic type of K & V
    // K is bound by the eq, hash and copy triats ie: K type must:
    // implement the copy trait, the eq trait and the hash trait
    impl<K: Eq + Hash + Copy, V> Data<K, V> {
        // new function impl of the data struct
        // this function returns self: ie a Data struct newly created
        pub fn new() -> Self {
            Data {
                cache: HashMap::new(),
            }
        }

        // this add function returns a result enum
        // this function adds to the data struct passed in and returns `Ok(())`
        pub fn add(&mut self, key: K, value: V) {
            self.cache.insert(key, value);
        }

        pub fn get(&self, key: K) -> Option<&V> {
            self.cache.get(&key)
        }

        pub fn delete(&mut self, key: K) -> Result<V, &str> {
            let deleted_data = self.cache.remove(&key);
            match deleted_data {
                Some(a) => Ok(a),
                None => return Err("Data not found"),
            }
        }

        pub fn update(&mut self, key: K, value: V) -> Result<&str, &str> {
            if self.cache.contains_key(&key) {
                self.cache.insert(key, value);
                // println!("successfully updated");
                Ok("Successfully updated")
            } else {
                Err("key you passed doesnt exist")
            }
        }
    }
}
