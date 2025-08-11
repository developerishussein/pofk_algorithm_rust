//! ✌️ Two Sum Using Set (Generic, Hashable, Additive)
//!
//! Returns true if there exist two distinct elements in the slice whose sum equals the target.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Copy` + `Sub<Output = T>` + `Add<Output = T>` + `PartialEq`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//! * `target` - The target sum.
//!
//! # Returns
//! * `bool` - True if such a pair exists, false otherwise.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::has_two_sum::has_two_sum;
//! let arr = [1, 2, 3, 4];
//! assert!(has_two_sum(&arr, 5));
//! assert!(!has_two_sum(&arr, 10));
//! ```
use std::collections::HashSet;
use std::ops::Sub;
pub fn has_two_sum<T>(slice: &[T], target: T) -> bool
where
    T: Eq + std::hash::Hash + Copy + Sub<Output = T> + std::ops::Add<Output = T> + PartialEq,
{
    let mut seen = HashSet::new();
    for &item in slice {
        let complement = target - item;
        if seen.contains(&complement) {
            return true;
        }
        seen.insert(item);
    }
    false
}
