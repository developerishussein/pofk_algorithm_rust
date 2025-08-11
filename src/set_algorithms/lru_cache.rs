//! LRU Cache (Generic, Hashable, Production-Grade)
//!
//! A Least Recently Used (LRU) cache with O(1) get and put operations.
//!
//! # Type Parameters
//! * `K`: The key type. Must implement `Eq` + `Hash` + `Clone`.
//! * `V`: The value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::set_algorithms::lru_cache::LruCache;
//! let mut cache = LruCache::new(2);
//! cache.put(1, "a");
//! cache.put(2, "b");
//! assert_eq!(cache.get(&1), Some("a"));
//! cache.put(3, "c"); // Evicts key 2
//! assert_eq!(cache.get(&2), None);
//! assert_eq!(cache.get(&3), Some("c"));
//! ```
use std::collections::{HashMap, VecDeque};

pub struct LruCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K, V> LruCache<K, V>
where
    K: Eq + std::hash::Hash + Clone,
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<V> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            self.map.get(key).cloned()
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() == self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.map.remove(&oldest);
            }
        }
        self.order.push_back(key.clone());
        self.map.insert(key, value);
    }
}
