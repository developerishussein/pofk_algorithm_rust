#[cfg(test)]
mod tests {
    use crate::string_algorithms::rabin_karp::rabin_karp;

    #[test]
    fn test_rabin_karp_basic() {
        assert_eq!(rabin_karp("hello world", "world"), Some(6));
        assert_eq!(rabin_karp("abc", "d"), None);
        assert_eq!(rabin_karp("a😊b😊c", "😊b"), Some(1));
    }

    #[test]
    fn test_rabin_karp_empty_needle() {
        assert_eq!(rabin_karp("abc", ""), Some(0));
    }

    #[test]
    fn test_rabin_karp_empty_haystack() {
        assert_eq!(rabin_karp("", "a"), None);
    }
}
