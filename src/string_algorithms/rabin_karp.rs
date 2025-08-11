//! Rabin-Karp Substring Search (Unicode-safe, Production-Grade)
//!
//! Searches for the first occurrence of a pattern in a string slice using the Rabin-Karp algorithm.
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
//! use pofk_algorithms::string_algorithms::rabin_karp::rabin_karp;
//! assert_eq!(rabin_karp("hello world", "world"), Some(6));
//! assert_eq!(rabin_karp("abc", "d"), None);
//! assert_eq!(rabin_karp("a😊b😊c", "😊b"), Some(1));
//! ```
pub fn rabin_karp(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() { return Some(0); }
    let h_len = haystack.chars().count();
    let n_len = needle.chars().count();
    if n_len > h_len { return None; }
    let base: u64 = 256;
    let modulus: u64 = 1_000_000_007;
    let h_chars: Vec<_> = haystack.chars().collect();
    let n_chars: Vec<_> = needle.chars().collect();
    let mut h_hash = 0u64;
    let mut n_hash = 0u64;
    let mut highest = 1u64;
    for i in 0..n_len {
        n_hash = (n_hash * base + n_chars[i] as u64) % modulus;
        h_hash = (h_hash * base + h_chars[i] as u64) % modulus;
        if i > 0 { highest = (highest * base) % modulus; }
    }
    for i in 0..=h_len - n_len {
        if h_hash == n_hash && h_chars[i..i + n_len] == n_chars[..] {
            return Some(h_chars[..i].iter().map(|c| c.len_utf8()).sum());
        }
        if i + n_len < h_len {
            h_hash = (h_hash + modulus - highest * h_chars[i] as u64 % modulus) % modulus;
            h_hash = (h_hash * base + h_chars[i + n_len] as u64) % modulus;
        }
    }
    None
}
