//! Longest Common Prefix (Unicode-safe, Production-Grade)
//!
//! Finds the longest common prefix among a slice of string slices.
//!
//! # Arguments
//! * `strs` - The slice of string slices.
//!
//! # Returns
//! * `String` - The longest common prefix. Returns an empty string if input is empty.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::string_algorithms::longest_common_prefix::longest_common_prefix;
//! let strs = ["flower", "flow", "flight"];
//! assert_eq!(longest_common_prefix(&strs), "fl");
//! let strs = ["dog", "racecar", "car"];
//! assert_eq!(longest_common_prefix(&strs), "");
//! ```
pub fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() { return String::new(); }
    let mut prefix = strs[0];
    for s in &strs[1..] {
        let mut i = 0;
        let min_len = prefix.chars().count().min(s.chars().count());
        let prefix_chars: Vec<_> = prefix.chars().collect();
        let s_chars: Vec<_> = s.chars().collect();
        while i < min_len && prefix_chars[i] == s_chars[i] {
            i += 1;
        }
        prefix = &prefix[..prefix_chars[..i].iter().map(|c| c.len_utf8()).sum::<usize>()];
        if prefix.is_empty() { break; }
    }
    prefix.to_string()
}
