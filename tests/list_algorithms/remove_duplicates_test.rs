#[cfg(test)]
mod tests {
    use crate::list_algorithms::remove_duplicates::remove_duplicates;

    #[test]
    fn test_remove_duplicates_basic() {
        let arr = [1, 2, 2, 3, 4, 4, 5];
        assert_eq!(remove_duplicates(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_duplicates_empty() {
        let arr: [i32; 0] = [];
        let unique = remove_duplicates(&arr);
        assert!(unique.is_empty());
    }

    #[test]
    fn test_remove_duplicates_single() {
        let arr = [42];
        let unique = remove_duplicates(&arr);
        assert_eq!(unique, vec![42]);
    }

    #[test]
    fn test_remove_duplicates_all_unique() {
        let arr = [1, 2, 3, 4, 5];
        let unique = remove_duplicates(&arr);
        assert_eq!(unique, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_duplicates_all_duplicates() {
        let arr = [2, 2, 2, 2];
        let unique = remove_duplicates(&arr);
        assert_eq!(unique, vec![2]);
    }
}
