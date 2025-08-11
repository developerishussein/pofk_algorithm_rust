//! Edit Distance (Levenshtein Distance, Generic, Production-Grade)
//!
//! Computes the minimum edit distance between two sequences.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `PartialEq` + `Copy`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::dp_algorithms::edit_distance::*;
//! let a = vec!['k', 'i', 't', 't', 'e', 'n'];
//! let b = vec!['s', 'i', 't', 't', 'i', 'n', 'g'];
//! assert_eq!(edit_distance(&a, &b), 3);
//! ```
pub fn edit_distance<T: PartialEq + Copy>(a: &[T], b: &[T]) -> usize {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m {
        dp[i][0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]);
            }
        }
    }
    dp[m][n]
}
