#[cfg(test)]
mod tests {
    use crate::list_algorithms::quick_sort::quick_sort;

    #[test]
    fn test_quick_sort_basic() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_quick_sort_empty() {
        let mut arr: [i32; 0] = [];
        quick_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_quick_sort_single() {
        let mut arr = [42];
        quick_sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_quick_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_duplicates() {
        let mut arr = [3, 1, 2, 3, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 3]);
    }
}
