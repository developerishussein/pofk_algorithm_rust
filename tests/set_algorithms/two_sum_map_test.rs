#[cfg(test)]
mod tests {
    use crate::set_algorithms::two_sum_map::two_sum_map;

    #[test]
    fn test_two_sum_map_basic() {
        let arr = [2, 7, 11, 15];
        assert_eq!(two_sum_map(&arr, 9), Some((0, 1)));
        assert_eq!(two_sum_map(&arr, 26), Some((2, 3)));
        assert_eq!(two_sum_map(&arr, 100), None);
    }

    #[test]
    fn test_two_sum_map_empty() {
        let arr: [i32; 0] = [];
        assert_eq!(two_sum_map(&arr, 0), None);
    }

    #[test]
    fn test_two_sum_map_duplicates() {
        let arr = [3, 3];
        assert_eq!(two_sum_map(&arr, 6), Some((0, 1)));
    }
}
