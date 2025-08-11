//! Reverse String (Unicode-safe, Production-Grade)
//!
//! Reverses a string slice, returning a new String. Handles Unicode correctly.
//!
//! # Arguments
//! * `s` - The string slice to reverse.
//!
//! # Returns
//! * `String` - The reversed string.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::string_algorithms::reverse_string::reverse_string;
//! assert_eq!(reverse_string("hello"), "olleh");
//! assert_eq!(reverse_string("a😊b"), "b😊a");
//! ```
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
