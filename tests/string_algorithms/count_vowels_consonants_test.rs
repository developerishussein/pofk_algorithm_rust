#[cfg(test)]
mod tests {
    use crate::string_algorithms::count_vowels_consonants::count_vowels_consonants;

    #[test]
    fn test_count_vowels_consonants_basic() {
        assert_eq!(count_vowels_consonants("hello world"), (3, 7));
        assert_eq!(count_vowels_consonants("a😊b"), (1, 1));
        assert_eq!(count_vowels_consonants("aeiouAEIOU"), (10, 0));
        assert_eq!(count_vowels_consonants("bcdfg"), (0, 5));
    }

    #[test]
    fn test_count_vowels_consonants_empty() {
        assert_eq!(count_vowels_consonants(""), (0, 0));
    }
}
