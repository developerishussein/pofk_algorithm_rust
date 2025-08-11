//! Palindrome Check (Unicode-safe, Production-Grade)
//!
//! Checks if a string slice is a palindrome, considering Unicode characters.
//!
//! # Arguments
//! * `s` - The string slice to check.
//!
//! # Returns
//! * `bool` - True if the string is a palindrome, false otherwise.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::string_algorithms::palindrome_check::palindrome_check;
//! assert!(palindrome_check("racecar"));
//! assert!(palindrome_check("a😊a"));
//! assert!(!palindrome_check("hello"));
//! ```
pub fn palindrome_check(s: &str) -> bool {
    let chars: Vec<_> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}
