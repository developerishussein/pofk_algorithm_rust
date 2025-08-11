//! House Robber (Generic, Production-Grade)
//!
//! Returns the maximum amount that can be robbed without robbing adjacent houses.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Copy` + `Ord` + `Add<Output = T>` + `Default`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::dp_algorithms::house_robber::*;
//! let nums = vec![1,2,3,1];
//! assert_eq!(house_robber(&nums), 4);
//! ```
use std::ops::Add;

pub fn house_robber<T>(nums: &[T]) -> T
where
    T: Copy + Ord + Add<Output = T> + Default,
{
    let n = nums.len();
    if n == 0 { return T::default(); }
    if n == 1 { return nums[0]; }
    let mut prev1 = T::default();
    let mut prev2 = T::default();
    for &num in nums {
        let temp = prev1;
        prev1 = prev1.max(prev2 + num);
        prev2 = temp;
    }
    prev1
}
