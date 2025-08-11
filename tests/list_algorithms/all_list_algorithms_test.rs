//! Corporate-level tests for all list algorithms in the library.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_algorithms::*;

    #[test]
    fn test_linear_search() {
        let arr = [10, 20, 30];
        assert_eq!(linear_search(&arr, &20), Some(1));
        assert_eq!(linear_search(&arr, &99), None);
    }

    #[test]
    fn test_binary_search() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, &7), Some(3));
        assert_eq!(binary_search(&arr, &2), None);
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = [64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, [11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [5, 2, 4, 6, 1, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_counting_sort() {
        let mut arr = [4u32, 2, 2, 8, 3, 3, 1];
        counting_sort(&mut arr, 8);
        assert_eq!(arr, [1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    fn test_reverse_list() {
        let mut arr = [1, 2, 3, 4, 5];
        reverse_list(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_find_max_min() {
        let arr = [3, 1, 4, 1, 5, 9];
        assert_eq!(find_max_min(&arr), Some((1, 9)));
        let empty: [i32; 0] = [];
        assert_eq!(find_max_min(&empty), None);
    }

    #[test]
    fn test_find_duplicates() {
        let arr = [1, 2, 2, 3, 4, 4, 4, 5];
        let mut dups = find_duplicates(&arr);
        dups.sort();
        assert_eq!(dups, vec![2, 4]);
    }

    #[test]
    fn test_kadane() {
        let arr = [1, -2, 3, 4, -1, 2, 1, -5, 4];
        assert_eq!(kadane(&arr), Some(9));
        let empty: [i32; 0] = [];
        assert_eq!(kadane(&empty), None);
    }

    #[test]
    fn test_sliding_window_max_sum() {
        let arr = [2, 1, 5, 1, 3, 2];
        assert_eq!(sliding_window_max_sum(&arr, 3), Some(9));
        assert_eq!(sliding_window_max_sum(&arr, 0), None);
    }

    #[test]
    fn test_remove_duplicates() {
        let arr = [1, 2, 2, 3, 4, 4, 5];
        assert_eq!(remove_duplicates(&arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_rotate_array_right() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        rotate_array_right(&mut arr, 3);
        assert_eq!(arr, [5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn test_prefix_sum() {
        let arr = [1, 2, 3, 4];
        assert_eq!(prefix_sum(&arr), vec![1, 3, 6, 10]);
    }
}
