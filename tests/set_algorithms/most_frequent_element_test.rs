#[cfg(test)]
mod tests {
    use crate::set_algorithms::most_frequent_element::most_frequent_element;

    #[test]
    fn test_most_frequent_element_basic() {
        let arr = [1, 2, 2, 3, 3, 3, 4];
        assert_eq!(most_frequent_element(&arr), Some(3));
    }

    #[test]
    fn test_most_frequent_element_tie() {
        let arr = [1, 2, 2, 1];
        let result = most_frequent_element(&arr);
        assert!(result == Some(1) || result == Some(2));
    }

    #[test]
    fn test_most_frequent_element_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(most_frequent_element(&arr), None);
    }

    #[test]
    fn test_most_frequent_element_strings() {
        let arr = ["a", "b", "a", "c", "b", "b"];
        assert_eq!(most_frequent_element(&arr), Some("b"));
    }
}
