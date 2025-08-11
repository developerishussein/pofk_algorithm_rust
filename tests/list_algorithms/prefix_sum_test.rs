#[cfg(test)]
mod tests {
    use crate::list_algorithms::prefix_sum::prefix_sum;

    #[test]
    fn test_prefix_sum_basic() {
        let arr = [1, 2, 3, 4];
        assert_eq!(prefix_sum(&arr), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_prefix_sum_empty() {
        let arr: [i32; 0] = [];
        let sums = prefix_sum(&arr);
        assert!(sums.is_empty());
    }

    #[test]
    fn test_prefix_sum_single() {
        let arr = [42];
        let sums = prefix_sum(&arr);
        assert_eq!(sums, vec![42]);
    }

    #[test]
    fn test_prefix_sum_negative() {
        let arr = [1, -2, 3, -4];
        assert_eq!(prefix_sum(&arr), vec![1, -1, 2, -2]);
    }
}
