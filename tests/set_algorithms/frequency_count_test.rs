#[cfg(test)]
mod tests {
    use crate::set_algorithms::frequency_count::frequency_count;
    use std::collections::HashMap;

    #[test]
    fn test_frequency_count_basic() {
        let arr = [1, 2, 2, 3, 1, 4];
        let freq = frequency_count(&arr);
        assert_eq!(freq.get(&1), Some(&2));
        assert_eq!(freq.get(&2), Some(&2));
        assert_eq!(freq.get(&3), Some(&1));
        assert_eq!(freq.get(&4), Some(&1));
    }

    #[test]
    fn test_frequency_count_empty() {
        let arr: [i32; 0] = [];
        let freq = frequency_count(&arr);
        assert!(freq.is_empty());
    }

    #[test]
    fn test_frequency_count_strings() {
        let arr = ["a", "b", "a", "c", "b", "b"];
        let freq = frequency_count(&arr);
        assert_eq!(freq.get(&"a"), Some(&2));
        assert_eq!(freq.get(&"b"), Some(&3));
        assert_eq!(freq.get(&"c"), Some(&1));
    }
}
