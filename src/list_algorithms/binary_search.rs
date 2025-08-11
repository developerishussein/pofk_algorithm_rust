//! 🔎 Binary Search (Generic)
//!
//! Searches for a target value in a sorted slice. Returns the index if found, otherwise None.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Arguments
//! * `slice` - The sorted slice to search.
//! * `target` - The value to search for.
//!
//! # Returns
//! * `Option<usize>` - The index of the target if found, or None.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::list_algorithms::binary_search::binary_search;
//! let idx = binary_search(&[1, 3, 5, 7, 9], &7);
//! assert_eq!(idx, Some(3));
//! ```
pub fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = slice.len();
    while low < high {
        let mid = low + (high - low) / 2;
        match slice[mid].cmp(target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
        }
    }
    None
}
