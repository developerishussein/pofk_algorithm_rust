#[cfg(test)]
mod tests {
    use crate::list_algorithms::linear_search::linear_search;

    #[test]
    fn test_linear_search_found() {
        let arr = [10, 20, 30];
        assert_eq!(linear_search(&arr, &20), Some(1));
    }

    #[test]
    fn test_linear_search_not_found() {
        let arr = [10, 20, 30];
        assert_eq!(linear_search(&arr, &99), None);
    }

    #[test]
    fn test_linear_search_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(linear_search(&arr, &1), None);
    }

    #[test]
    fn test_linear_search_first() {
        let arr = [7, 8, 9];
        assert_eq!(linear_search(&arr, &7), Some(0));
    }

    #[test]
    fn test_linear_search_last() {
        let arr = [7, 8, 9];
        assert_eq!(linear_search(&arr, &9), Some(2));
    }

    #[test]
    fn test_linear_search_duplicates() {
        let arr = [1, 2, 2, 3];
        assert_eq!(linear_search(&arr, &2), Some(1));
    }
}
