//! 🪟 Unique Sliding Window (Generic, Hashable)
//!
//! Returns true if there exists a window of size `k` in the slice with all unique elements.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//! * `k` - The window size.
//!
//! # Returns
//! * `bool` - True if such a window exists, false otherwise.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::has_unique_window::has_unique_window;
//! let arr = [1, 2, 3, 1, 4, 5];
//! assert!(has_unique_window(&arr, 3));
//! assert!(has_unique_window(&arr, 4));
//! ```
use std::collections::HashSet;
pub fn has_unique_window<T: Eq + std::hash::Hash>(slice: &[T], k: usize) -> bool {
    if k == 0 || slice.len() < k { return false; }
    let mut set = HashSet::new();
    for i in 0..=slice.len() - k {
        set.clear();
        for j in 0..k {
            set.insert(&slice[i + j]);
        }
        if set.len() == k {
            return true;
        }
    }
    false
}
