#[cfg(test)]
mod tests {
    use crate::list_algorithms::kadanes_algorithm::kadane;

    #[test]
    fn test_kadane_basic() {
        let arr = [1, -2, 3, 4, -1, 2, 1, -5, 4];
        assert_eq!(kadane(&arr), Some(9));
    }

    #[test]
    fn test_kadane_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(kadane(&arr), None);
    }

    #[test]
    fn test_kadane_all_negative() {
        let arr = [-3, -1, -4, -2];
        assert_eq!(kadane(&arr), Some(-1));
    }

    #[test]
    fn test_kadane_all_positive() {
        let arr = [1, 2, 3, 4];
        assert_eq!(kadane(&arr), Some(10));
    }

    #[test]
    fn test_kadane_single() {
        let arr = [42];
        assert_eq!(kadane(&arr), Some(42));
    }
}
