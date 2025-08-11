//! 🏆 Find Max/Min (Generic)
//!
//! Finds the maximum and minimum values in a slice.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord` + `Copy`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//!
//! # Returns
//! * `Option<(T, T)>` - A tuple of (min, max) if the slice is non-empty, otherwise None.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::find_max_min::find_max_min;
//! let arr = [3, 1, 4, 1, 5, 9];
//! assert_eq!(find_max_min(&arr), Some((1, 9)));
//! ```
pub fn find_max_min<T: Ord + Copy>(slice: &[T]) -> Option<(T, T)> {
    if slice.is_empty() { return None; }
    let mut min = slice[0];
    let mut max = slice[0];
    for &item in slice.iter().skip(1) {
        if item < min { min = item; }
        if item > max { max = item; }
    }
    Some((min, max))
}
