//! 🔄 Reverse List (Generic, In-Place)
//!
//! Reverses a mutable slice in place.
//!
//! # Type Parameters
//! * `T`: The element type.
//!
//! # Arguments
//! * `slice` - The mutable slice to reverse.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::reverse_list::reverse_list;
//! let mut arr = [1, 2, 3, 4, 5];
//! reverse_list(&mut arr);
//! assert_eq!(arr, [5, 4, 3, 2, 1]);
//! ```
pub fn reverse_list<T>(slice: &mut [T]) {
    let n = slice.len();
    for i in 0..n / 2 {
        slice.swap(i, n - 1 - i);
    }
}
