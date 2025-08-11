//! Corporate-grade, generic, and highly optimized list algorithms for Rust.
//!
//! # Overview
//! This module provides a suite of high-performance, production-ready algorithms for searching and sorting lists, as well as common list operations. All algorithms are generic over type `T` and leverage Rust's trait system for maximum flexibility and safety.
//!
//! ## Algorithms Included
//! - Linear Search
//! - Binary Search
//! - Bubble Sort
//! - Selection Sort
//! - Insertion Sort
//! - Merge Sort
//! - Quick Sort
//! - Counting Sort
//! - Reverse List
//! - Find Max/Min
//! - Find Duplicates
//! - Kadane's Algorithm (Maximum Subarray Sum)
//! - Sliding Window
//! - Two Pointers
//! - Remove Duplicates
//! - Rotate Array
//! - Prefix Sum
//!
//! # Usage
//! ```rust
//! use pofk_algorithms::list_algorithms::linear_search::linear_search;
//! let idx = linear_search(&[1, 2, 3], &2);
//! ```

pub mod linear_search;
pub mod binary_search;
pub mod bubble_sort;
pub mod selection_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;
pub mod counting_sort;
pub mod reverse_list;
pub mod find_max_min;
pub mod find_duplicates;
pub mod kadanes_algorithm;
pub mod sliding_window;
pub mod remove_duplicates;
pub mod rotate_array_right;
pub mod prefix_sum;
