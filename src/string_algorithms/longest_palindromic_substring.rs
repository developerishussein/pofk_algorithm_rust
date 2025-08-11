//! Longest Palindromic Substring (Unicode-safe, Production-Grade)
//!
//! Finds the longest palindromic substring in a string slice.
//!
//! # Arguments
//! * `s` - The string slice to search.
//!
//! # Returns
//! * `String` - The longest palindromic substring. Returns an empty string if input is empty.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::string_algorithms::longest_palindromic_substring::longest_palindromic_substring;
//! assert_eq!(longest_palindromic_substring("babad"), "bab"); // or "aba"
//! assert_eq!(longest_palindromic_substring("cbbd"), "bb");
//! assert_eq!(longest_palindromic_substring("a"), "a");
//! ```
pub fn longest_palindromic_substring(s: &str) -> String {
    let chars: Vec<_> = s.chars().collect();
    let n = chars.len();
    if n == 0 { return String::new(); }
    let (mut start, mut max_len) = (0, 1);
    for i in 0..n {
        // Odd length
        let (mut l, mut r) = (i as isize, i as isize);
        while l >= 0 && r < n as isize && chars[l as usize] == chars[r as usize] {
            if (r - l + 1) as usize > max_len {
                start = l as usize;
                max_len = (r - l + 1) as usize;
            }
            l -= 1;
            r += 1;
        }
        // Even length
        let (mut l, mut r) = (i as isize, i as isize + 1);
        while l >= 0 && r < n as isize && chars[l as usize] == chars[r as usize] {
            if (r - l + 1) as usize > max_len {
                start = l as usize;
                max_len = (r - l + 1) as usize;
            }
            l -= 1;
            r += 1;
        }
    }
    chars[start..start + max_len].iter().collect()
}
