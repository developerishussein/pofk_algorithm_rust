//! Edit Distance (Levenshtein Distance, Unicode-safe, Production-Grade)
//!
//! Computes the minimum edit distance between two string slices.
//!
//! # Arguments
//! * `a` - The first string slice.
//! * `b` - The second string slice.
//!
//! # Returns
//! * `usize` - The minimum number of operations required to convert `a` into `b`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::string_algorithms::edit_distance::edit_distance;
//! assert_eq!(edit_distance("kitten", "sitting"), 3);
//! assert_eq!(edit_distance("flaw", "lawn"), 2);
//! assert_eq!(edit_distance("", "abc"), 3);
//! ```
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<_> = a.chars().collect();
    let b_chars: Vec<_> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }
    for i in 1..=m {
        for j in 1..=n {
            if a_chars[i - 1] == b_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j - 1].min(dp[i][j - 1]).min(dp[i - 1][j]);
            }
        }
    }
    dp[m][n]
}
