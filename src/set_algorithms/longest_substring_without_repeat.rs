//! Longest Substring Without Repeat (Set + Map, Production-Grade)
//!
//! Finds the length of the longest substring without repeating characters.
//!
//! # Arguments
//! * `s` - The input string slice.
//!
//! # Returns
//! * `usize` - The length of the longest substring without repeats.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::longest_substring_without_repeat::longest_substring_without_repeat;
//! assert_eq!(longest_substring_without_repeat("abcabcbb"), 3);
//! assert_eq!(longest_substring_without_repeat("bbbbb"), 1);
//! assert_eq!(longest_substring_without_repeat("pwwkew"), 3);
//! ```
use std::collections::HashMap;

pub fn longest_substring_without_repeat(s: &str) -> usize {
    let mut map = HashMap::new();
    let (mut max_len, mut start) = (0, 0);
    for (i, c) in s.chars().enumerate() {
        if let Some(&prev) = map.get(&c) {
            start = start.max(prev + 1);
        }
        map.insert(c, i);
        max_len = max_len.max(i - start + 1);
    }
    max_len
}
