//! Count Vowels and Consonants (Unicode-aware, Production-Grade)
//!
//! Counts the number of vowels and consonants in a string slice.
//!
//! # Arguments
//! * `s` - The string slice to analyze.
//!
//! # Returns
//! * `(usize, usize)` - (vowel_count, consonant_count)
//!
//! # Example
//! ```rust
//! use pofk_algorithm::string_algorithms::count_vowels_consonants::count_vowels_consonants;
//! assert_eq!(count_vowels_consonants("hello world"), (3, 7));
//! assert_eq!(count_vowels_consonants("a😊b"), (1, 1));
//! ```
pub fn count_vowels_consonants(s: &str) -> (usize, usize) {
    let vowels = [
        'a', 'e', 'i', 'o', 'u',
        'A', 'E', 'I', 'O', 'U',
    ];
    let mut vowel_count = 0;
    let mut consonant_count = 0;
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            if vowels.contains(&c) {
                vowel_count += 1;
            } else {
                consonant_count += 1;
            }
        }
    }
    (vowel_count, consonant_count)
}
