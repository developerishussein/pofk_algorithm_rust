#[cfg(test)]
mod tests {
    use crate::list_algorithms::counting_sort::counting_sort;

    #[test]
    fn test_counting_sort_basic() {
        let mut arr = [4u32, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr, 8u32);
        assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn test_counting_sort_empty() {
        let mut arr: [u32; 0] = [];
        counting_sort(&mut arr, 0u32);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_counting_sort_single() {
        let mut arr = [42u32];
        counting_sort(&mut arr, 42u32);
        assert_eq!(arr, [42u32]);
    }

    #[test]
    fn test_counting_sort_sorted() {
        let mut arr = [1u32, 2, 3, 4, 5];
        counting_sort(&mut arr, 5u32);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_counting_sort_reverse() {
        let mut arr = [5u32, 4, 3, 2, 1];
        counting_sort(&mut arr, 5u32);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_counting_sort_duplicates() {
        let mut arr = [3u32, 1, 2, 3, 1];
        counting_sort(&mut arr, 3u32);
        assert_eq!(arr, [1, 1, 2, 3, 3]);
    }
}
