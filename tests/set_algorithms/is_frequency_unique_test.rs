#[cfg(test)]
mod tests {
    use crate::set_algorithms::is_frequency_unique::is_frequency_unique;
    #[test]
    fn test_is_frequency_unique() {
        let arr = [1, 2, 2, 3, 3, 3];
        assert!(is_frequency_unique(&arr));
        let arr = [1, 2, 2, 3, 3];
        assert!(!is_frequency_unique(&arr));
        let arr: [i32; 0] = [];
        assert!(is_frequency_unique(&arr));
    }
}
