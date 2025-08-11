//! 🆙 Frequency Uniqueness (Generic, Hashable)
//!
//! Returns true if all element frequencies in the slice are unique.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash`.
//!
//! # Arguments
//! * `slice` - The slice to check.
//!
//! # Returns
//! * `bool` - True if all frequencies are unique, false otherwise.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::is_frequency_unique::is_frequency_unique;
//! let arr = [1, 2, 2, 3, 3, 3];
//! assert!(is_frequency_unique(&arr));
//! let arr = [1, 2, 2, 3, 3];
//! assert!(!is_frequency_unique(&arr));
//! ```
use std::collections::{HashMap, HashSet};
pub fn is_frequency_unique<T: Eq + std::hash::Hash>(slice: &[T]) -> bool {
    let mut freq = HashMap::new();
    for item in slice {
        *freq.entry(item).or_insert(0) += 1;
    }
    let mut seen = HashSet::new();
    for &count in freq.values() {
        if !seen.insert(count) {
            return false;
        }
    }
    true
}
