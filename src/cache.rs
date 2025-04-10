use std::hash::Hash;
use std::{collections::HashMap, str};

// This is a struct `Data` with generic types K & V
// this struct stores a cache field which has a hash map of
// Key of generic type K and value of generic type V
struct Data<K, V> {
    cache: HashMap<K, V>,
}

// implementation for the struct Data, this is taking a generic type of K & V
// K is bound by the eq, hash and copy triats ie: K type must:
// implement the copy trait, the eq trait and the hash trait
impl<K: Eq + Hash + Copy, V> Data<K, V> {
    // new function impl of the data struct
    // this function returns self: ie a Data struct newly created
    fn new() -> Self {
        Data {
            cache: HashMap::new(),
        }
    }

    // this add function returns a result enum
    // this function adds to the data struct passed in and returns `Ok(())`
    fn add(&mut self, key: K, value: V) -> Result<(), _> {
        self.cache.insert(key, value);
        Ok(())
    }

    fn get(&self, key: K) -> Option<&V> {
        self.cache.get(&key)
    }

    fn delete(&mut self, key: K) -> Result<V, &str> {
        let deleted_data = self.cache.remove(&key);
        match deleted_data {
            Some(a) => Ok(a),
            None => return Err("Data not found"),
        }
    }

    fn update(&mut self, key: K, value: V) -> Result<(), &str> {
        if self.cache.contains_key(&key) {
            self.cache.insert(key, value);
            println!("successfully updated");
            Ok(())
        } else {
            Err("key you passed doesnt exist")
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {}
