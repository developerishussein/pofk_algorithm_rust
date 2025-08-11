//! Rod Cutting (Generic, Production-Grade)
//!
//! Returns the maximum obtainable value by cutting up the rod and selling the pieces.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Copy` + `Add<Output = T>` + `Ord` + `Default`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::dp_algorithms::rod_cutting::*;
//! let prices = vec![1, 5, 8, 9, 10, 17, 17, 20];
//! let n = 8;
//! assert_eq!(rod_cutting(&prices, n), 22);
//! ```
use std::ops::Add;

pub fn rod_cutting<T>(prices: &[T], n: usize) -> T
where
    T: Copy + Add<Output = T> + Ord + Default,
{
    let mut dp = vec![T::default(); n + 1];
    for i in 1..=n {
        let mut max_val = T::default();
        for j in 0..i {
            max_val = max_val.max(prices[j] + dp[i - j - 1]);
        }
        dp[i] = max_val;
    }
    dp[n]
}
