//! 🔗 Two Pointers (Generic)
//!
//! Finds two indices such that their values sum to a target (for sorted slices).
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord + Copy + std::ops::Add<Output = T>`.
//!
//! # Arguments
//! * `slice` - The sorted slice to search.
//! * `target` - The target sum.
//!
//! # Returns
//! * `Option<(usize, usize)>` - The indices of the two elements, or None if not found.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::list_algorithms::two_pointers_sum;
//! let arr = [1, 2, 3, 4, 6];
//! assert_eq!(two_pointers_sum(&arr, 6), Some((1, 3)));
//! ```
pub fn two_pointers_sum<T>(slice: &[T], target: T) -> Option<(usize, usize)>
where
    T: Ord + Copy + std::ops::Add<Output = T>,
{
    let (mut left, mut right) = (0, slice.len().saturating_sub(1));
    while left < right {
        let sum = slice[left] + slice[right];
        if sum == target {
            return Some((left, right));
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    None
}
