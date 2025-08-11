//! 🔍 Linear Search (Generic)
//!
//! Searches for a target value in a slice. Returns the index if found, otherwise None.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `PartialEq`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//! * `target` - The value to search for.
//!
//! # Returns
//! * `Option<usize>` - The index of the target if found, or None.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::linear_search::linear_search;
//! let idx = linear_search(&[10, 20, 30], &20);
//! assert_eq!(idx, Some(1));
//! ```
pub fn linear_search<T: PartialEq>(slice: &[T], target: &T) -> Option<usize> {
    for (i, item) in slice.iter().enumerate() {
        if item == target {
            return Some(i);
        }
    }
    None
}
