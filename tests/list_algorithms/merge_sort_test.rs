#[cfg(test)]
mod tests {
    use crate::list_algorithms::merge_sort::merge_sort;

    #[test]
    fn test_merge_sort_basic() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_merge_sort_single() {
        let mut arr = vec![42];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_merge_sort_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_duplicates() {
        let mut arr = vec![3, 1, 2, 3, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3]);
    }
}
