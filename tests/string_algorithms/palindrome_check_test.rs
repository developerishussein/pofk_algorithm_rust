#[cfg(test)]
mod tests {
    use crate::string_algorithms::palindrome_check::palindrome_check;

    #[test]
    fn test_palindrome_ascii() {
        assert!(palindrome_check("racecar"));
        assert!(!palindrome_check("hello"));
    }

    #[test]
    fn test_palindrome_unicode() {
        assert!(palindrome_check("a😊a"));
        assert!(!palindrome_check("a😊b"));
    }

    #[test]
    fn test_palindrome_empty() {
        assert!(palindrome_check(""));
    }

    #[test]
    fn test_palindrome_single_char() {
        assert!(palindrome_check("x"));
    }
}
