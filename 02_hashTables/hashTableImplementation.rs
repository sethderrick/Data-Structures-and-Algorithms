/// A simple hash table implementation in Rust.
///
/// This hash table uses separate chaining to handle hash collisions.
/// Each bucket at a given index in the vector can hold multiple key-value pairs that share the same hash.
struct HashTable<K, V> {
    /// The internal storage for the hash table, where each position can store a vector of tuples (key, value).
    data: Vec<Option<Vec<(K, V)>>>,
}

impl<K, V> HashTable<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    /// Creates a new hash table with the specified size.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of buckets in the hash table.
    pub fn new(size: usize) -> Self {
        HashTable {
            data: vec![None; size],
        }
    }

    /// Generates a hash index based on the key to determine where to store the data.
    /// Typically, this method runs in O(1) average time complexity.
    ///
    /// # Arguments
    ///
    /// * `key` - The key used to calculate the hash.
    fn hash(&self, key: &K) -> usize {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.data.len()
    }

    /// Inserts a key-value pair into the hash table.
    /// This operation runs in O(1) average time complexity because it appends the element at the end of a vector.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert into the hash table.
    /// * `value` - The value associated with the key.
    pub fn set(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        match self.data[index].as_mut() {
            Some(bucket) => bucket.push((key, value)),
            None => self.data[index] = Some(vec![(key, value)]),
        }
    }

    /// Retrieves a value from the hash table based on the given key.
    /// This method runs in O(1) average time complexity as it performs a single linear search on a small subset of items.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve the value for.
    ///
    /// # Returns
    ///
    /// Returns an option containing the value if the key exists, or `None` if the key does not exist.
    pub fn get(&self, key: &K) -> Option<V> {
        let index = self.hash(key);
        self.data[index].as_ref().and_then(|bucket| {
            bucket
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.clone())
        })
    }

    /// Returns a vector of all keys stored in the hash table.
    /// This operation has O(n) time complexity, where n is the number of keys stored in the hash table.
    ///
    /// # Returns
    ///
    /// A vector containing all keys.
    pub fn keys(&self) -> Vec<K> {
        let mut keys = Vec::new();
        for bucket in self.data.iter().filter_map(|b| b.as_ref()) {
            for (key, _) in bucket {
                keys.push(key.clone());
            }
        }
        keys
    }
}

fn main() {
    let mut my_hash_table = HashTable::new(50);
    my_hash_table.set("grapes", 10000);
    my_hash_table.set("apples", 54);
    my_hash_table.set("oranges", 2);

    if let Some(value) = my_hash_table.get(&"grapes") {
        println!("Value for 'grapes': {}", value);
    }

    let keys = my_hash_table.keys();
    println!("Keys in my hash table: {:?}", keys);
}
