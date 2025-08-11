//! Partition Equal Subset Sum (Generic, Production-Grade)
//!
//! Determines if the array can be partitioned into two subsets with equal sum.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Copy` + `Add<Output = T>` + `From<u8>` + `Ord` + `PartialEq`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::dp_algorithms::partition_equal_subset_sum::*;
//! let nums = vec![1usize, 5, 11, 5];
//! assert!(partition_equal_subset_sum(&nums));
//! ```
use std::ops::Add;

pub fn partition_equal_subset_sum<T>(nums: &[T]) -> bool
where
    T: Copy + Add<Output = T> + From<u8> + Ord + PartialEq + Into<usize>,
{
    let sum: T = nums.iter().fold(T::from(0u8), |acc, &x| acc + x);
    if <T as Into<usize>>::into(sum) % 2 != 0 {
        return false;
    }
    let target = T::from((<T as Into<usize>>::into(sum) / 2) as u8);
    let mut dp = vec![false; (<T as Into<usize>>::into(target)) + 1];
    dp[0] = true;
    for &num in nums {
        for j in (<T as Into<usize>>::into(num)..=<T as Into<usize>>::into(target)).rev() {
            dp[j] = dp[j] || dp[j - <T as Into<usize>>::into(num)];
        }
    }
    dp[<T as Into<usize>>::into(target)]
}
