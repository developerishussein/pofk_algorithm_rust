#[cfg(test)]
mod tests {
    use crate::list_algorithms::binary_search::binary_search;

    #[test]
    fn test_binary_search_found() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &7), Some(3));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &2), None);
    }

    #[test]
    fn test_binary_search_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, &1), None);
    }

    #[test]
    fn test_binary_search_first() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, &1), Some(0));
    }

    #[test]
    fn test_binary_search_last() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, &5), Some(4));
    }

    #[test]
    fn test_binary_search_duplicates() {
        let arr = [1, 2, 2, 2, 3];
        let idx = binary_search(&arr, &2);
        assert!(idx == Some(1) || idx == Some(2) || idx == Some(3));
    }
}
