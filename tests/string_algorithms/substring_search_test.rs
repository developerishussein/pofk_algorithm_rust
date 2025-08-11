#[cfg(test)]
mod tests {
    use crate::string_algorithms::substring_search::substring_search;

    #[test]
    fn test_substring_search_basic() {
        assert_eq!(substring_search("hello world", "world"), Some(6));
        assert_eq!(substring_search("abc", "d"), None);
        assert_eq!(substring_search("a😊b😊c", "😊b"), Some(1));
    }

    #[test]
    fn test_substring_search_empty_needle() {
        assert_eq!(substring_search("abc", ""), Some(0));
    }

    #[test]
    fn test_substring_search_empty_haystack() {
        assert_eq!(substring_search("", "a"), None);
    }
}
