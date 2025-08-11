//! ➕ Prefix Sum (Generic, Additive)
//!
//! Computes the prefix sum of a slice, returning a new Vec of the same length.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Copy + std::ops::Add<Output = T>`.
//!
//! # Arguments
//! * `slice` - The slice to compute prefix sums for.
//!
//! # Returns
//! * `Vec<T>` - The prefix sums.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::list_algorithms::prefix_sum::prefix_sum;
//! let arr = [1, 2, 3, 4];
//! let sums = prefix_sum(&arr);
//! assert_eq!(sums, vec![1, 3, 6, 10]);
//! ```
pub fn prefix_sum<T>(slice: &[T]) -> Vec<T>
where
    T: Copy + std::ops::Add<Output = T>,
{
    let mut result = Vec::with_capacity(slice.len());
    let mut sum: Option<T> = None;
    for &item in slice {
        sum = Some(match sum {
            Some(s) => s + item,
            None => item,
        });
        result.push(sum.unwrap());
    }
    result
}
