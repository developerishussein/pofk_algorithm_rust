#[cfg(test)]
mod tests {
    use crate::string_algorithms::longest_palindromic_substring::longest_palindromic_substring;

    #[test]
    fn test_longest_palindromic_substring_basic() {
        let res = longest_palindromic_substring("babad");
        assert!(res == "bab" || res == "aba");
        assert_eq!(longest_palindromic_substring("cbbd"), "bb");
        assert_eq!(longest_palindromic_substring("a"), "a");
        assert_eq!(longest_palindromic_substring(""), "");
    }

    #[test]
    fn test_longest_palindromic_substring_unicode() {
        assert_eq!(longest_palindromic_substring("a😊a"), "a😊a");
        assert_eq!(longest_palindromic_substring("ab😊ba"), "ab😊ba");
    }
}
