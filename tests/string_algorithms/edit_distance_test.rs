#[cfg(test)]
mod tests {
    use crate::string_algorithms::edit_distance::edit_distance;

    #[test]
    fn test_edit_distance_basic() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("flaw", "lawn"), 2);
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", ""), 3);
        assert_eq!(edit_distance("", ""), 0);
    }

    #[test]
    fn test_edit_distance_unicode() {
        assert_eq!(edit_distance("a😊b", "a😊c"), 1);
        assert_eq!(edit_distance("😊", "a"), 1);
    }
}
