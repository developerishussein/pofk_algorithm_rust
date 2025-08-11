#[cfg(test)]
mod tests {
    use crate::list_algorithms::sliding_window::sliding_window_max_sum;

    #[test]
    fn test_sliding_window_basic() {
        let arr = [2, 1, 5, 1, 3, 2];
        assert_eq!(sliding_window_max_sum(&arr, 3), Some(9));
    }

    #[test]
    fn test_sliding_window_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(sliding_window_max_sum(&arr, 3), None);
    }

    #[test]
    fn test_sliding_window_k_zero() {
        let arr = [1, 2, 3];
        assert_eq!(sliding_window_max_sum(&arr, 0), None);
    }

    #[test]
    fn test_sliding_window_k_greater_than_len() {
        let arr = [1, 2, 3];
        assert_eq!(sliding_window_max_sum(&arr, 5), None);
    }

    #[test]
    fn test_sliding_window_k_one() {
        let arr = [1, 2, 3];
        assert_eq!(sliding_window_max_sum(&arr, 1), Some(3));
    }
}
