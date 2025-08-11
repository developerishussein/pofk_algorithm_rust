//! Two Sum (Map Version, Generic, Hashable)
//!
//! Finds indices of the two numbers in the slice that add up to the target, using a map for O(n) performance.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Copy` + `Eq` + `Hash` + `std::ops::Sub<Output = T>`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//! * `target` - The target sum.
//!
//! # Returns
//! * `Option<(usize, usize)>` - Indices of the two elements, or None if not found.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::two_sum_map::two_sum_map;
//! let arr = [2, 7, 11, 15];
//! assert_eq!(two_sum_map(&arr, 9), Some((0, 1)));
//! assert_eq!(two_sum_map(&arr, 26), Some((2, 3)));
//! assert_eq!(two_sum_map(&arr, 100), None);
//! ```
use std::collections::HashMap;

pub fn two_sum_map<T>(slice: &[T], target: T) -> Option<(usize, usize)>
where
    T: Copy + Eq + std::hash::Hash + std::ops::Sub<Output = T>,
{
    let mut map = HashMap::with_capacity(slice.len());
    for (i, &num) in slice.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}
