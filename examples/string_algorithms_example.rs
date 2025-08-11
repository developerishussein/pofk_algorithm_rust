//! Example usage of advanced string algorithms from the pofk_algorithms library.
//!
//! This file demonstrates how to use each string algorithm with idiomatic, production-grade Rust code.

use pofk_algorithms::string_algorithms::{
    reverse_string::reverse_string,
    palindrome_check::palindrome_check,
    anagram_check::anagram_check,
    longest_palindromic_substring::longest_palindromic_substring,
    string_compression::string_compression,
    substring_search::substring_search,
    rabin_karp::rabin_karp,
    longest_common_prefix::longest_common_prefix,
    edit_distance::edit_distance,
    count_vowels_consonants::count_vowels_consonants,
};

fn main() {
    // Reverse String
    println!("Reverse: {}", reverse_string("hello"));
    println!("Reverse (unicode): {}", reverse_string("a😊b"));

    // Palindrome Check
    println!("Palindrome (racecar): {}", palindrome_check("racecar"));
    println!("Palindrome (hello): {}", palindrome_check("hello"));

    // Anagram Check
    println!("Anagram (listen, silent): {}", anagram_check("listen", "silent"));
    println!("Anagram (hello, world): {}", anagram_check("hello", "world"));

    // Longest Palindromic Substring
    println!("Longest palindromic substring (babad): {}", longest_palindromic_substring("babad"));
    println!("Longest palindromic substring (cbbd): {}", longest_palindromic_substring("cbbd"));

    // String Compression
    println!("String compression (aabcccccaaa): {}", string_compression("aabcccccaaa"));
    println!("String compression (abc): {}", string_compression("abc"));

    // Substring Search (Brute Force)
    println!("Substring search (hello world, world): {:?}", substring_search("hello world", "world"));
    println!("Substring search (abc, d): {:?}", substring_search("abc", "d"));

    // Rabin-Karp
    println!("Rabin-Karp (hello world, world): {:?}", rabin_karp("hello world", "world"));
    println!("Rabin-Karp (abc, d): {:?}", rabin_karp("abc", "d"));

    // Longest Common Prefix
    let strs = ["flower", "flow", "flight"];
    println!("Longest common prefix: {}", longest_common_prefix(&strs));
    let strs = ["dog", "racecar", "car"];
    println!("Longest common prefix: {}", longest_common_prefix(&strs));

    // Edit Distance
    println!("Edit distance (kitten, sitting): {}", edit_distance("kitten", "sitting"));
    println!("Edit distance (flaw, lawn): {}", edit_distance("flaw", "lawn"));

    // Count Vowels/Consonants
    println!("Vowels/Consonants (hello world): {:?}", count_vowels_consonants("hello world"));
    println!("Vowels/Consonants (a😊b): {:?}", count_vowels_consonants("a😊b"));
}
