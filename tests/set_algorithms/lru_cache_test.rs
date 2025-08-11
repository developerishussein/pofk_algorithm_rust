#[cfg(test)]
mod tests {
    use crate::set_algorithms::lru_cache::LruCache;

    #[test]
    fn test_lru_cache_basic() {
        let mut cache = LruCache::new(2);
        cache.put(1, "a");
        cache.put(2, "b");
        assert_eq!(cache.get(&1), Some("a"));
        cache.put(3, "c"); // Evicts key 2
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some("c"));
    }

    #[test]
    fn test_lru_cache_update() {
        let mut cache = LruCache::new(2);
        cache.put(1, "a");
        cache.put(2, "b");
        cache.put(1, "z");
        assert_eq!(cache.get(&1), Some("z"));
        cache.put(3, "c");
        assert_eq!(cache.get(&2), None);
    }

    #[test]
    fn test_lru_cache_capacity_one() {
        let mut cache = LruCache::new(1);
        cache.put(1, "a");
        cache.put(2, "b");
        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some("b"));
    }
}
