//! Palindromic Substrings (Generic, Production-Grade)
//!
//! Counts the number of palindromic substrings in a sequence.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `PartialEq` + `Copy`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::dp_algorithms::palindromic_substrings::*;
//! let s = vec!['a', 'b', 'a'];
//! assert_eq!(palindromic_substrings(&s), 4);
//! ```
pub fn palindromic_substrings<T: PartialEq + Copy>(s: &[T]) -> usize {
    let n = s.len();
    let mut count = 0;
    let mut dp = vec![vec![false; n]; n];
    for i in 0..n {
        dp[i][i] = true;
        count += 1;
    }
    for l in 2..=n {
        for i in 0..=n-l {
            let j = i + l - 1;
            if s[i] == s[j] && (l == 2 || dp[i+1][j-1]) {
                dp[i][j] = true;
                count += 1;
            }
        }
    }
    count
}
