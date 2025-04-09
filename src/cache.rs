use std::fmt::Error;
use std::{collections::HashMap, str};
use std::hash::Hash;

struct Data<K, V>(HashMap<K, V>);

impl<K: Eq + Hash + Copy, V> Data<K, V> {
    
    fn new() -> HashMap<K, V> {
        // , key: i32, value: &str
        let local_cache: HashMap<K, V> = HashMap::new();
        local_cache
    }

    fn add(&mut self, key: K, value: V) {
        let _current_key = &self.0.entry(key).or_insert(value);
    }
    // String::from("john")
    // get
    fn get(&self, key: K) -> Option<&V> {
        self.0.get(&key)
    }

    fn delete(&mut self, key: K) -> Result<V, &str> {
        // type Output = &str;
        let deleted_data = self.0.remove(&key);
        match deleted_data {
            Some(a) => {
               Ok(a)
            }
            None => return Err("Data not found"),
        }
    }

    fn update(&mut self, key: K, value: V){
        let current_value = &self.get(key);
        match current_value {
            Some(_) => { let _ = &self.0.insert(key, value);
            println!("successfully updated") },
            None => println!("key you passed doesnt exist")
        }

    }
}

fn main() {
    let global_hashmap: HashMap<i32, &str> = Data::new();
}

#[cfg(test)]
mod test {
    
}