//! String Compression (Run-Length Encoding, Production-Grade)
//!
//! Compresses a string using run-length encoding. Only compresses if the result is shorter.
//!
//! # Arguments
//! * `s` - The string slice to compress.
//!
//! # Returns
//! * `String` - The compressed string, or the original if compression does not reduce length.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::string_algorithms::string_compression::string_compression;
//! assert_eq!(string_compression("aabcccccaaa"), "a2b1c5a3");
//! assert_eq!(string_compression("abc"), "abc");
//! ```
pub fn string_compression(s: &str) -> String {
    if s.is_empty() { return String::new(); }
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        let mut count = 1;
        while let Some(&next) = chars.peek() {
            if next == c {
                chars.next();
                count += 1;
            } else {
                break;
            }
        }
        result.push(c);
        result.push_str(&count.to_string());
    }
    if result.len() < s.len() { result } else { s.to_string() }
}
