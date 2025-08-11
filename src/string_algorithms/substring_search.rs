//! Substring Search (Brute Force, Unicode-safe, Production-Grade)
//!
//! Searches for the first occurrence of a pattern in a string slice. Returns the starting index if found.
//!
//! # Arguments
//! * `haystack` - The string to search in.
//! * `needle` - The pattern to search for.
//!
//! # Returns
//! * `Option<usize>` - The starting index of the first occurrence, or None if not found.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::string_algorithms::substring_search::substring_search;
//! assert_eq!(substring_search("hello world", "world"), Some(6));
//! assert_eq!(substring_search("abc", "d"), None);
//! assert_eq!(substring_search("a😊b😊c", "😊b"), Some(1));
//! ```
pub fn substring_search(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() { return Some(0); }
    let h_len = haystack.chars().count();
    let n_len = needle.chars().count();
    if n_len > h_len { return None; }
    let h_chars: Vec<_> = haystack.chars().collect();
    let n_chars: Vec<_> = needle.chars().collect();
    for i in 0..=h_len - n_len {
        if h_chars[i..i + n_len] == n_chars[..] {
            return Some(h_chars[..i].iter().map(|c| c.len_utf8()).sum());
        }
    }
    None
}
