#[cfg(test)]
mod tests {
    use crate::list_algorithms::find_duplicates::find_duplicates;

    #[test]
    fn test_find_duplicates_basic() {
        let arr = [1, 2, 2, 3, 4, 4, 4, 5];
        let mut dups = find_duplicates(&arr);
        dups.sort();
        assert_eq!(dups, vec![2, 4]);
    }

    #[test]
    fn test_find_duplicates_empty() {
        let arr: [i32; 0] = [];
        let dups = find_duplicates(&arr);
        assert!(dups.is_empty());
    }

    #[test]
    fn test_find_duplicates_single() {
        let arr = [42];
        let dups = find_duplicates(&arr);
        assert!(dups.is_empty());
    }

    #[test]
    fn test_find_duplicates_all_unique() {
        let arr = [1, 2, 3, 4, 5];
        let dups = find_duplicates(&arr);
        assert!(dups.is_empty());
    }

    #[test]
    fn test_find_duplicates_all_duplicates() {
        let arr = [2, 2, 2, 2];
        let dups = find_duplicates(&arr);
        assert_eq!(dups, vec![2]);
    }
}
