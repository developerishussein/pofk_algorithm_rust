#[cfg(test)]
mod tests {
    use crate::string_algorithms::longest_common_prefix::longest_common_prefix;

    #[test]
    fn test_longest_common_prefix_basic() {
        let strs = ["flower", "flow", "flight"];
        assert_eq!(longest_common_prefix(&strs), "fl");
        let strs = ["dog", "racecar", "car"];
        assert_eq!(longest_common_prefix(&strs), "");
    }

    #[test]
    fn test_longest_common_prefix_empty() {
        let strs: [&str; 0] = [];
        assert_eq!(longest_common_prefix(&strs), "");
    }

    #[test]
    fn test_longest_common_prefix_single() {
        let strs = ["alone"];
        assert_eq!(longest_common_prefix(&strs), "alone");
    }
}
