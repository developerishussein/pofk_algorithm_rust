//! Anagram Check (Unicode-safe, Production-Grade)
//!
//! Checks if two string slices are anagrams (contain the same characters with the same frequencies).
//!
//! # Arguments
//! * `a` - The first string slice.
//! * `b` - The second string slice.
//!
//! # Returns
//! * `bool` - True if the strings are anagrams, false otherwise.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::string_algorithms::anagram_check::anagram_check;
//! assert!(anagram_check("listen", "silent"));
//! assert!(!anagram_check("hello", "world"));
//! assert!(anagram_check("a😊b", "b😊a"));
//! ```
use std::collections::HashMap;

pub fn anagram_check(a: &str, b: &str) -> bool {
    if a.chars().count() != b.chars().count() {
        return false;
    }
    let mut freq_a = HashMap::new();
    let mut freq_b = HashMap::new();
    for c in a.chars() {
        *freq_a.entry(c).or_insert(0) += 1;
    }
    for c in b.chars() {
        *freq_b.entry(c).or_insert(0) += 1;
    }
    freq_a == freq_b
}
