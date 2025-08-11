#[cfg(test)]
mod tests {
    use crate::set_algorithms::anagram_checker::anagram_checker;

    #[test]
    fn test_anagram_checker_true() {
        let a = ["a", "b", "c"];
        let b = ["c", "b", "a"];
        assert!(anagram_checker(&a, &b));
    }

    #[test]
    fn test_anagram_checker_false() {
        let a = ["a", "b", "c"];
        let b = ["a", "b", "b"];
        assert!(!anagram_checker(&a, &b));
    }

    #[test]
    fn test_anagram_checker_empty() {
        let a: [i32; 0] = [];
        let b: [i32; 0] = [];
        assert!(anagram_checker(&a, &b));
    }

    #[test]
    fn test_anagram_checker_diff_length() {
        let a = [1, 2, 3];
        let b = [1, 2];
        assert!(!anagram_checker(&a, &b));
    }
}
