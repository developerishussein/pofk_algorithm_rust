//! Frequency Count (Generic, Hashable)
//!
//! Counts the frequency of each element in the input slice.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `slice` - The slice to count frequencies for.
//!
//! # Returns
//! * `HashMap<T, usize>` - A map from element to its frequency.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::frequency_count::frequency_count;
//! let arr = [1, 2, 2, 3, 1, 4];
//! let freq = frequency_count(&arr);
//! assert_eq!(freq.get(&1), Some(&2));
//! assert_eq!(freq.get(&2), Some(&2));
//! assert_eq!(freq.get(&3), Some(&1));
//! assert_eq!(freq.get(&4), Some(&1));
//! ```
use std::collections::HashMap;

pub fn frequency_count<T: Eq + std::hash::Hash + Clone>(slice: &[T]) -> HashMap<T, usize> {
    let mut freq = HashMap::with_capacity(slice.len());
    for item in slice {
        *freq.entry(item.clone()).or_insert(0) += 1;
    }
    freq
}
