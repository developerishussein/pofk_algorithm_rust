use pofk_algorithms::dp_algorithms::palindromic_substrings::palindromic_substrings;

#[test]
fn test_palindromic_substrings_large() {
    let s: Vec<char> = "aabaa".chars().collect();
    assert_eq!(palindromic_substrings(&s), 9);
    let s: Vec<char> = "a".repeat(100).chars().collect();
    assert_eq!(palindromic_substrings(&s), 5050);
}
