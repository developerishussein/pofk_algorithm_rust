#[cfg(test)]
mod tests {
    use crate::set_algorithms::has_duplicates::has_duplicates;
    #[test]
    fn test_has_duplicates() {
        let arr = [1, 2, 3, 2];
        assert!(has_duplicates(&arr));
        let arr = [1, 2, 3];
        assert!(!has_duplicates(&arr));
        let arr: [i32; 0] = [];
        assert!(!has_duplicates(&arr));
        let arr = [1];
        assert!(!has_duplicates(&arr));
    }
}
