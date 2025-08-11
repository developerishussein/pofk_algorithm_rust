#[cfg(test)]
mod tests {
    use crate::string_algorithms::anagram_check::anagram_check;

    #[test]
    fn test_anagram_true() {
        assert!(anagram_check("listen", "silent"));
        assert!(anagram_check("a😊b", "b😊a"));
    }

    #[test]
    fn test_anagram_false() {
        assert!(!anagram_check("hello", "world"));
        assert!(!anagram_check("abc", "ab"));
    }

    #[test]
    fn test_anagram_empty() {
        assert!(anagram_check("", ""));
    }
}
