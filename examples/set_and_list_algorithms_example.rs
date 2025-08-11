//! Example usage of advanced set and list algorithms from the pofk_algorithms library.
//!
//! This file demonstrates how to use each algorithm with idiomatic, production-grade Rust code.

use pofk_algorithms::list_algorithms::{
    find_duplicates::find_duplicates,
    prefix_sum::prefix_sum,
    remove_duplicates::remove_duplicates,
    rotate_array_right::rotate_array_right,
    sliding_window::sliding_window_max_sum,
    kadanes_algorithm::kadane,
};
use pofk_algorithms::set_algorithms::{
    has_duplicates::has_duplicates,
    find_union::find_union,
    find_intersection::find_intersection,
    set_difference::set_difference,
    is_frequency_unique::is_frequency_unique,
    has_two_sum::has_two_sum,
    has_unique_window::has_unique_window,
    frequency_count::frequency_count,
    grouping_elements::grouping_elements,
    first_non_repeated::first_non_repeated,
    anagram_checker::anagram_checker,
    two_sum_map::two_sum_map,
    lru_cache::LruCache,
    most_frequent_element::most_frequent_element,
    top_k_frequent::top_k_frequent,
    longest_substring_without_repeat::longest_substring_without_repeat,
};

fn main() {
    // List Algorithms
    let arr = [1, 2, 2, 3, 4, 4, 4, 5];
    let mut dups = find_duplicates(&arr);
    dups.sort();
    println!("Duplicates: {:?}", dups);

    let arr = [1, 2, 3, 4];
    let sums = prefix_sum(&arr);
    println!("Prefix sums: {:?}", sums);

    let arr = [1, 2, 2, 3, 4, 4, 5];
    let mut unique = remove_duplicates(&arr);
    unique.sort();
    println!("Unique elements: {:?}", unique);

    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    rotate_array_right(&mut arr, 3);
    println!("Rotated array: {:?}", arr);

    let arr = [2, 1, 5, 1, 3, 2];
    let max_sum = sliding_window_max_sum(&arr, 3);
    println!("Max sliding window sum (k=3): {:?}", max_sum);

    let arr = [1, -2, 3, 4, -1, 2, 1, -5, 4];
    let max_subarray = kadane(&arr);
    println!("Max subarray sum (Kadane): {:?}", max_subarray);

    // Set Algorithms
    let arr = [1, 2, 3, 2];
    println!("Has duplicates: {}", has_duplicates(&arr));

    let a = [1, 2, 3];
    let b = [2, 3, 4];
    let mut union = find_union(&a, &b);
    union.sort();
    println!("Union: {:?}", union);

    let mut intersection = find_intersection(&a, &b);
    intersection.sort();
    println!("Intersection: {:?}", intersection);

    let mut diff = set_difference(&a, &b);
    diff.sort();
    println!("Set difference (a - b): {:?}", diff);

    let arr = [1, 2, 2, 3, 4, 4, 5];
    println!("Is frequency unique: {}", is_frequency_unique(&arr));

    let arr = [1, 2, 3, 4];
    println!("Has two sum (5): {}", has_two_sum(&arr, 5));
    println!("Has two sum (10): {}", has_two_sum(&arr, 10));

    let arr = [1, 2, 3, 1, 4, 5];
    println!("Has unique window (k=3): {}", has_unique_window(&arr, 3));
    println!("Has unique window (k=4): {}", has_unique_window(&arr, 4));

    // New Advanced Algorithms
    let arr = [1, 2, 2, 3, 1, 4];
    let freq = frequency_count(&arr);
    println!("Frequency count: {:?}", freq);

    let arr = ["apple", "apricot", "banana", "blueberry"];
    let groups = grouping_elements(&arr, |s| s.chars().next().unwrap());
    println!("Grouping elements by first char: {:?}", groups);

    let arr = ['a', 'b', 'c', 'a', 'b', 'd'];
    println!("First non-repeated: {:?}", first_non_repeated(&arr));

    let a = ["a", "b", "c"];
    let b = ["c", "b", "a"];
    println!("Anagram checker (true): {}", anagram_checker(&a, &b));
    let c = ["a", "b", "b"];
    println!("Anagram checker (false): {}", anagram_checker(&a, &c));

    let arr = [2, 7, 11, 15];
    println!("Two sum map (9): {:?}", two_sum_map(&arr, 9));
    println!("Two sum map (100): {:?}", two_sum_map(&arr, 100));

    let mut cache = LruCache::new(2);
    cache.put(1, "a");
    cache.put(2, "b");
    println!("LRU get 1: {:?}", cache.get(&1));
    cache.put(3, "c");
    println!("LRU get 2 (should be None): {:?}", cache.get(&2));
    println!("LRU get 3: {:?}", cache.get(&3));

    let arr = [1, 2, 2, 3, 3, 3, 4];
    println!("Most frequent element: {:?}", most_frequent_element(&arr));

    let arr = [1, 1, 1, 2, 2, 3];
    let mut top = top_k_frequent(&arr, 2);
    top.sort();
    println!("Top 2 frequent: {:?}", top);

    println!("Longest substring without repeat (\"abcabcbb\"): {}", longest_substring_without_repeat("abcabcbb"));
    println!("Longest substring without repeat (\"bbbbb\"): {}", longest_substring_without_repeat("bbbbb"));
    println!("Longest substring without repeat (\"pwwkew\"): {}", longest_substring_without_repeat("pwwkew"));
}
