//! First Non-Repeated Character (Generic, Hashable)
//!
//! Finds the first non-repeated element in a slice.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//!
//! # Returns
//! * `Option<T>` - The first non-repeated element, or None if all are repeated.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::set_algorithms::first_non_repeated::first_non_repeated;
//! let arr = ['a', 'b', 'c', 'a', 'b', 'd'];
//! assert_eq!(first_non_repeated(&arr), Some('c'));
//! ```
use std::collections::HashMap;

pub fn first_non_repeated<T: Eq + std::hash::Hash + Clone>(slice: &[T]) -> Option<T> {
    let mut freq = HashMap::with_capacity(slice.len());
    for item in slice {
        *freq.entry(item).or_insert(0) += 1;
    }
    for item in slice {
        if freq[item] == 1 {
            return Some(item.clone());
        }
    }
    None
}
