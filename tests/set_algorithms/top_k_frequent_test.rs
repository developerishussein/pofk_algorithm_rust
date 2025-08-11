#[cfg(test)]
mod tests {
    use crate::set_algorithms::top_k_frequent::top_k_frequent;

    #[test]
    fn test_top_k_frequent_basic() {
        let arr = [1, 1, 1, 2, 2, 3];
        let mut top = top_k_frequent(&arr, 2);
        top.sort();
        assert_eq!(top, vec![1, 2]);
    }

    #[test]
    fn test_top_k_frequent_k_greater_than_unique() {
        let arr = [1, 2];
        let mut top = top_k_frequent(&arr, 5);
        top.sort();
        assert_eq!(top, vec![1, 2]);
    }

    #[test]
    fn test_top_k_frequent_empty() {
        let arr: [i32; 0] = [];
        let top = top_k_frequent(&arr, 2);
        assert!(top.is_empty());
    }

    #[test]
    fn test_top_k_frequent_strings() {
        let arr = ["a", "b", "a", "c", "b", "b"];
        let mut top = top_k_frequent(&arr, 1);
        top.sort();
        assert_eq!(top, vec!["b"]);
    }
}
