//! Longest Increasing Subsequence (LIS, Generic, Production-Grade)
//!
//! Finds the length of the longest increasing subsequence in a sequence.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `PartialOrd` + `Copy`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::dp_algorithms::longest_increasing_subsequence::*;
//! let arr = vec![10, 9, 2, 5, 3, 7, 101, 18];
//! assert_eq!(longest_increasing_subsequence(&arr), 4);
//! ```
pub fn longest_increasing_subsequence<T: PartialOrd + Copy>(arr: &[T]) -> usize {
    if arr.is_empty() { return 0; }
    let mut dp = vec![1; arr.len()];
    for i in 1..arr.len() {
        for j in 0..i {
            if arr[i] > arr[j] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1;
            }
        }
    }
    *dp.iter().max().unwrap()
}
