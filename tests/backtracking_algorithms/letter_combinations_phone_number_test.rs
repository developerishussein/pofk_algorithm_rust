use pofk_algorithms::backtracking_algorithms::letter_combinations_phone_number::letter_combinations;

#[test]
fn test_letter_combinations_large() {
    let digits = "23";
    let result = letter_combinations(digits);
    assert_eq!(result.len(), 9);
    let digits = "";
    let result = letter_combinations(digits);
    assert!(result.is_empty());
}
