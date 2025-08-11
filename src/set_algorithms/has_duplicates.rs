//! 🔍 Detect Duplicates in a Collection (Generic, Hashable)
//!
//! Returns true if the collection contains any duplicate elements.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash`.
//!
//! # Arguments
//! * `slice` - The slice to check for duplicates.
//!
//! # Returns
//! * `bool` - True if duplicates exist, false otherwise.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::set_algorithms::has_duplicates::has_duplicates;
//! let arr = [1, 2, 3, 2];
//! assert!(has_duplicates(&arr));
//! ```
use std::collections::HashSet;
pub fn has_duplicates<T: Eq + std::hash::Hash>(slice: &[T]) -> bool {
    let mut seen = HashSet::new();
    for item in slice {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}
