pub struct HashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> HashTable<K, V> {
    pub fn new() -> Self {
        let buckets = (0..16).map(|_| Vec::new()).collect();
        HashTable { buckets }
    }
}
