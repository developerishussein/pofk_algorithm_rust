//! 🧩 Find Duplicates (Generic, Hashable)
//!
//! Returns a vector of all duplicate elements in the slice.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `slice` - The slice to search for duplicates.
//!
//! # Returns
//! * `Vec<T>` - A vector containing all duplicates (each duplicate appears once).
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::find_duplicates::find_duplicates;
//! let arr = [1, 2, 2, 3, 4, 4, 4, 5];
//! let mut dups = find_duplicates(&arr);
//! dups.sort();
//! let mut expected = vec![2, 4];
//! expected.sort();
//! assert_eq!(dups, expected);
//! ```
use std::collections::HashSet;
pub fn find_duplicates<T: Eq + std::hash::Hash + Clone>(slice: &[T]) -> Vec<T> {
    let mut seen = HashSet::new();
    let mut duplicates = HashSet::new();
    for item in slice {
        if !seen.insert(item) {
            duplicates.insert(item.clone());
        }
    }
    duplicates.into_iter().collect()
}
