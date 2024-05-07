struct HashTable {
    data: Vec<Option<(String, i32)>>,
}

impl HashTable {
    fn new(size: usize) -> Self {
        HashTable {
            data: vec![None; size],
        }
    }

    fn _hash(&self, key: &str) -> usize {
        let mut hash = 0;
        for (i, c) in key.chars().enumerate() {
            hash = (hash + c as usize * i) % self.data.len();
        }
        hash
    }

    fn set(&mut self, key: String, value: i32) {
        let index = self._hash(&key);
        self.data[index] = Some((key, value));
    }

    fn get(&self, key: &str) -> Option<i32> {
        let index = self._hash(key);
        if let Some((_, value)) = &self.data[index] {
            Some(*value)
        } else {
            None
        }
    }
}

fn main() {
    let mut my_hash_table = HashTable::new(50);
    my_hash_table.set(String::from("grapes"), 10000);
    println!("{:?}", my_hash_table.get("grapes")); // Output: Some(10000)
    my_hash_table.set(String::from("apples"), 9);
    println!("{:?}", my_hash_table.get("apples")); // Output: Some(9)
}
