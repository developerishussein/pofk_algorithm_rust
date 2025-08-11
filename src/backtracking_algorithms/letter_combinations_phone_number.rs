//! Letter Combinations of a Phone Number (Production-Grade)
//!
//! Returns all possible letter combinations that the number could represent.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::backtracking_algorithms::letter_combinations_phone_number::*;
//! let digits = "23";
//! let result = letter_combinations(digits);
//! assert_eq!(result, vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
//! ```
pub fn letter_combinations(digits: &str) -> Vec<String> {
    if digits.is_empty() { return vec![]; }
    let map = [
        "",    // 0
        "",    // 1
        "abc", // 2
        "def", // 3
        "ghi", // 4
        "jkl", // 5
        "mno", // 6
        "pqrs",// 7
        "tuv", // 8
        "wxyz" // 9
    ];
    let mut res = Vec::new();
    let mut comb = String::new();
    fn backtrack(digits: &[u8], idx: usize, map: &[&str; 10], comb: &mut String, res: &mut Vec<String>) {
        if idx == digits.len() {
            res.push(comb.clone());
            return;
        }
        let digit = (digits[idx] - b'0') as usize;
        for c in map[digit].chars() {
            comb.push(c);
            backtrack(digits, idx + 1, map, comb, res);
            comb.pop();
        }
    }
    backtrack(digits.as_bytes(), 0, &map, &mut comb, &mut res);
    res
}
