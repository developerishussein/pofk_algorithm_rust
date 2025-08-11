//! Subset Sum (Generic, Production-Grade)
//!
//! Determines if there is a subset of the given set with a sum equal to a given value.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Copy` + `Add<Output = T>` + `PartialEq` + `From<u8>` + `Ord`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::dp_algorithms::subset_sum::*;
//! let set = vec![3usize, 34, 4, 12, 5, 2];
//! assert!(subset_sum(&set, 9usize));
//! ```
use std::ops::Add;

pub fn subset_sum<T>(set: &[T], sum: T) -> bool
where
    T: Copy + Add<Output = T> + PartialEq + From<u8> + Ord + Into<usize>,
{
    let n = set.len();
    let sum_usize = <T as Into<usize>>::into(sum);
    let mut dp = vec![vec![false; sum_usize + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = true;
    }
    for i in 1..=n {
        for j in 1..=sum_usize {
            let set_elem_usize = <T as Into<usize>>::into(set[i - 1]);
            if set_elem_usize > j {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - set_elem_usize];
            }
        }
    }
    dp[n][sum_usize]
}
