#[cfg(test)]
mod tests {
    use crate::set_algorithms::longest_substring_without_repeat::longest_substring_without_repeat;

    #[test]
    fn test_longest_substring_without_repeat_basic() {
        assert_eq!(longest_substring_without_repeat("abcabcbb"), 3);
        assert_eq!(longest_substring_without_repeat("bbbbb"), 1);
        assert_eq!(longest_substring_without_repeat("pwwkew"), 3);
    }

    #[test]
    fn test_longest_substring_without_repeat_empty() {
        assert_eq!(longest_substring_without_repeat(""), 0);
    }

    #[test]
    fn test_longest_substring_without_repeat_all_unique() {
        assert_eq!(longest_substring_without_repeat("abcdef"), 6);
    }

    #[test]
    fn test_longest_substring_without_repeat_unicode() {
        assert_eq!(longest_substring_without_repeat("a😊b😊c"), 3);
    }
}
