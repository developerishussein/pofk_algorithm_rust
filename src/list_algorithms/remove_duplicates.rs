//! 🗑️ Remove Duplicates (Generic, Hashable)
//!
//! Removes duplicates from a slice, returning a new Vec with unique elements in order.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `slice` - The slice to deduplicate.
//!
//! # Returns
//! * `Vec<T>` - A vector with unique elements, preserving order.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::remove_duplicates::remove_duplicates;
//! let arr = [1, 2, 2, 3, 4, 4, 5];
//! let unique = remove_duplicates(&arr);
//! assert_eq!(unique, vec![1, 2, 3, 4, 5]);
//! ```
use std::collections::HashSet;
pub fn remove_duplicates<T: Eq + std::hash::Hash + Clone>(slice: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    for item in slice {
        if seen.insert(item) {
            result.push(item.clone());
        }
    }
    result
}
