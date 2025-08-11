#[cfg(test)]
mod tests {
    use crate::string_algorithms::reverse_string::reverse_string;

    #[test]
    fn test_reverse_ascii() {
        assert_eq!(reverse_string("hello"), "olleh");
    }

    #[test]
    fn test_reverse_unicode() {
        assert_eq!(reverse_string("a😊b"), "b😊a");
    }

    #[test]
    fn test_reverse_empty() {
        assert_eq!(reverse_string(""), "");
    }
}
