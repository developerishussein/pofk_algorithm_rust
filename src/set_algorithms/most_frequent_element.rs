//! Most Frequent Element (Generic, Hashable)
//!
//! Finds the element with the highest frequency in a slice.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//!
//! # Returns
//! * `Option<T>` - The most frequent element, or None if the slice is empty.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::most_frequent_element::most_frequent_element;
//! let arr = [1, 2, 2, 3, 3, 3, 4];
//! assert_eq!(most_frequent_element(&arr), Some(3));
//! ```
use std::collections::HashMap;

pub fn most_frequent_element<T: Eq + std::hash::Hash + Clone>(slice: &[T]) -> Option<T> {
    let mut freq = HashMap::with_capacity(slice.len());
    let mut max_count = 0;
    let mut result = None;
    for item in slice {
        let count = freq.entry(item).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            result = Some(item.clone());
        }
    }
    result
}
