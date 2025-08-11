//! 💰 Kadane's Algorithm (Maximum Subarray Sum)
//!
//! Finds the maximum sum of a contiguous subarray.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `PartialOrd + Copy + num_traits::Zero + std::ops::Add<Output = T>`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//!
//! # Returns
//! * `Option<T>` - The maximum subarray sum, or None if the slice is empty.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::list_algorithms::kadanes_algorithm::kadane;
//! let arr = [1, -2, 3, 4, -1, 2, 1, -5, 4];
//! assert_eq!(kadane(&arr), Some(9));
//! ```
use num_traits::Zero;
pub fn kadane<T>(slice: &[T]) -> Option<T>
where
    T: PartialOrd + Copy + Zero + std::ops::Add<Output = T>,
{
    if slice.is_empty() { return None; }
    let mut max_sum = slice[0];
    let mut curr_sum = slice[0];
    for &item in &slice[1..] {
        curr_sum = if curr_sum + item > item { curr_sum + item } else { item };
        if curr_sum > max_sum {
            max_sum = curr_sum;
        }
    }
    Some(max_sum)
}
