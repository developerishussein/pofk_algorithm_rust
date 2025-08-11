#[cfg(test)]
mod tests {
    use crate::set_algorithms::has_two_sum::has_two_sum;
    #[test]
    fn test_has_two_sum() {
        let arr = [1, 2, 3, 4];
        assert!(has_two_sum(&arr, 5));
        assert!(!has_two_sum(&arr, 10));
        let arr = [2, 2, 2];
        assert!(has_two_sum(&arr, 4));
        let arr: [i32; 0] = [];
        assert!(!has_two_sum(&arr, 0));
    }
}
