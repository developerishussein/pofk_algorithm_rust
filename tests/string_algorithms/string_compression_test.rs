#[cfg(test)]
mod tests {
    use crate::string_algorithms::string_compression::string_compression;

    #[test]
    fn test_string_compression_basic() {
        assert_eq!(string_compression("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(string_compression("abc"), "abc");
    }

    #[test]
    fn test_string_compression_empty() {
        assert_eq!(string_compression(""), "");
    }

    #[test]
    fn test_string_compression_unicode() {
        assert_eq!(string_compression("😊😊😊a"), "😊3a1");
        assert_eq!(string_compression("a😊"), "a😊");
    }
}
