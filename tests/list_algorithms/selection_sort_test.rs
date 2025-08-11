#[cfg(test)]
mod tests {
    use crate::list_algorithms::selection_sort::selection_sort;

    #[test]
    fn test_selection_sort_basic() {
        let mut arr = [64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut arr: [i32; 0] = [];
        selection_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_selection_sort_single() {
        let mut arr = [42];
        selection_sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_selection_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_duplicates() {
        let mut arr = [3, 1, 2, 3, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 3]);
    }
}
