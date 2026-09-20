use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct HashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> HashTable<K, V> {
    pub fn new() -> Self {
        let buckets = (0..16).map(|_| Vec::new()).collect();
        HashTable { buckets }
    }

    fn bucket_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.bucket_index(&key);
        for (stored_key, stored_value) in &mut self.buckets[index] {
            if stored_key == &key {
                *stored_value = value;
                return;
            }
        }
        self.buckets[index].push((key, value));
    }
}
