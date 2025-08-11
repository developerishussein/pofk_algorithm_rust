//! Longest Common Subsequence (LCS, Generic, Production-Grade)
//!
//! Finds the length of the longest common subsequence between two sequences.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `PartialEq` + `Copy`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::dp_algorithms::longest_common_subsequence::*;
//! let a = vec!['A', 'B', 'C', 'D', 'G', 'H'];
//! let b = vec!['A', 'E', 'D', 'F', 'H', 'R'];
//! assert_eq!(longest_common_subsequence(&a, &b), 3);
//! ```
pub fn longest_common_subsequence<T: PartialEq + Copy>(a: &[T], b: &[T]) -> usize {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}
