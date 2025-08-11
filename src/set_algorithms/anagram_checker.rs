//! Anagram Checker (Generic, Hashable)
//!
//! Checks if two slices are anagrams (contain the same elements with the same frequencies).
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash`.
//!
//! # Arguments
//! * `a` - The first slice.
//! * `b` - The second slice.
//!
//! # Returns
//! * `bool` - True if the slices are anagrams, false otherwise.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::anagram_checker::anagram_checker;
//! let a = ["a", "b", "c"];
//! let b = ["c", "b", "a"];
//! assert!(anagram_checker(&a, &b));
//! let c = ["a", "b", "b"];
//! assert!(!anagram_checker(&a, &c));
//! ```
use std::collections::HashMap;

pub fn anagram_checker<T: Eq + std::hash::Hash>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut freq_a = HashMap::with_capacity(a.len());
    let mut freq_b = HashMap::with_capacity(b.len());
    for item in a {
        *freq_a.entry(item).or_insert(0) += 1;
    }
    for item in b {
        *freq_b.entry(item).or_insert(0) += 1;
    }
    freq_a == freq_b
}
