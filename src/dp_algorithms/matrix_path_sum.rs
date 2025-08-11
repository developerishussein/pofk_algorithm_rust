//! Matrix Path Sum (Generic, Production-Grade)
//!
//! Finds the minimum path sum from top-left to bottom-right in a matrix.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Copy` + `Add<Output = T>` + `Ord` + `Default`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::dp_algorithms::matrix_path_sum::*;
//! let matrix = vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]];
//! assert_eq!(matrix_path_sum(&matrix), 7);
//! ```
use std::ops::Add;

pub fn matrix_path_sum<T>(matrix: &[Vec<T>]) -> T
where
    T: Copy + Add<Output = T> + Ord + Default,
{
    let m = matrix.len();
    let n = if m > 0 { matrix[0].len() } else { 0 };
    if m == 0 || n == 0 { return T::default(); }
    let mut dp = vec![vec![T::default(); n]; m];
    dp[0][0] = matrix[0][0];
    for i in 1..m {
        dp[i][0] = dp[i-1][0] + matrix[i][0];
    }
    for j in 1..n {
        dp[0][j] = dp[0][j-1] + matrix[0][j];
    }
    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i-1][j].min(dp[i][j-1]) + matrix[i][j];
        }
    }
    dp[m-1][n-1]
}
