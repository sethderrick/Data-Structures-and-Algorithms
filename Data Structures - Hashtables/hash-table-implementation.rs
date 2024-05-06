/*

*/

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A Hashtable struct that uses separate chaining for collision resolution.
/// The load factor is dynamically managed to optimize access times.
pub struct Hashtable {
    buckets: Vec<Vec<(String, i32)>>,
    items_count: usize,
    load_factor_threshold: f64, // Load factor threshold for resizing
}

impl Hashtable {
    /// Constructs a new, empty `Hashtable` with an initial bucket size and load factor threshold.
    pub fn new() -> Self {
        let initial_buckets = 16; // Initial size
        Hashtable {
            buckets: vec![Vec::new(); initial_buckets],
            items_count: 0,
            load_factor_threshold: 0.75, // Default load factor threshold
        }
    }

    /// Hashes a key into a usize index corresponding to the bucket location.
    fn hash(&self, key: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    /// Inserts a key-value pair into the hashtable.
    /// If the key already exists, updates its value.
    /// Checks if the hashtable needs resizing before adding a new item. 
    pub fn put(&mut self, key: String, value: i32) {
        if self.load_factor() > self.load_factor_threshold {
            self.resize();
        }
        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];

        for pair in bucket.iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        bucket.push((key, value));
        self.items_count += 1;
    }

    /// Returns the value corresponding to the key.
    pub fn get(&self, key: &str) -> Option<i32> {
        let index = self.hash(key);
        let bucket = &self.buckets[index];

        bucket.iter().find(|pair| pair.0 == key).map(|pair| pair.1)
    }

    /// Removes a key from the hashtable, if it exists.
    pub fn remove(&mut self, key: &str) {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];
        let initial_len = bucket.len();
        bucket.retain(|pair| pair.0 != key);
        if bucket.len() < initial_len {
            self.items_count -= 1;
        }
    }

    /// Calculates the current load factor of the hashtable.
    fn load_factor(&self) -> f64 {
        self.items_count as f64 / self.buckets.len() as f64
    }

    /// Resizes the buckets array by reallocating all items 
    /// into a new, larger bucket array and rehashing all existing keys.
    fn resize(&mut self) {
        let new_size = self.buckets.len() * 2; // Double the number of buckets
        let mut new_buckets = vec![Vec::new(); new_size];
        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);

        for bucket in old_buckets {
            for (key, value) in bucket {
                let index = self.hash(&key);
                self.buckets[index].push((key, value));
            }
        }
    }
}

impl std::fmt::Display for Hashtable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (index, bucket) in self.buckets.iter().enumerate() {
            write!(f, "Bucket {}: ", index)?;
            for (k, v) in bucket {
                write!(f, "({}, {}) ", k, v)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut h = Hashtable::new();
    h.put("grapes".to_string(), 1000);
    h.put("apples".to_string(), 10);
    h.put("ora".to_string(), 300);
    h.put("banan".to_string(), 200);
    println!("grapes: {:?}", h.get("grapes"));
    println!("{}", h);
    h.remove("apples");
    println!("{}", h);
}


// TODO: add tests