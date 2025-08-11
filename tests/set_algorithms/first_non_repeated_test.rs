#[cfg(test)]
mod tests {
    use crate::set_algorithms::first_non_repeated::first_non_repeated;

    #[test]
    fn test_first_non_repeated_basic() {
        let arr = ['a', 'b', 'c', 'a', 'b', 'd'];
        assert_eq!(first_non_repeated(&arr), Some('c'));
    }

    #[test]
    fn test_first_non_repeated_none() {
        let arr = ['a', 'a', 'b', 'b'];
        assert_eq!(first_non_repeated(&arr), None);
    }

    #[test]
    fn test_first_non_repeated_empty() {
        let arr: [char; 0] = [];
        assert_eq!(first_non_repeated(&arr), None);
    }

    #[test]
    fn test_first_non_repeated_numbers() {
        let arr = [1, 2, 2, 3, 1, 4];
        assert_eq!(first_non_repeated(&arr), Some(3));
    }
}
