#[cfg(test)]
mod tests {
    use crate::set_algorithms::has_unique_window::has_unique_window;
    #[test]
    fn test_has_unique_window() {
        let arr = [1, 2, 3, 1, 4, 5];
        assert!(has_unique_window(&arr, 3));
        assert!(!has_unique_window(&arr, 4));
        let arr = [1, 2, 3, 4, 5];
        assert!(has_unique_window(&arr, 5));
        let arr: [i32; 0] = [];
        assert!(!has_unique_window(&arr, 1));
    }
}
