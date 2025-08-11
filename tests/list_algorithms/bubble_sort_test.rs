#[cfg(test)]
mod tests {
    use crate::list_algorithms::bubble_sort::bubble_sort;

    #[test]
    fn test_bubble_sort_basic() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_bubble_sort_empty() {
        let mut arr: [i32; 0] = [];
        bubble_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_bubble_sort_single() {
        let mut arr = [42];
        bubble_sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_bubble_sort_sorted() {
        let mut arr = [1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_reverse() {
        let mut arr = [5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_bubble_sort_duplicates() {
        let mut arr = [3, 1, 2, 3, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 3]);
    }
}
