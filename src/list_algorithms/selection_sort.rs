//! 🔽 Selection Sort (Generic, In-Place)
//!
//! Sorts a mutable slice in ascending order using the selection sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Arguments
//! * `slice` - The mutable slice to sort.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::selection_sort::selection_sort;
//! let mut arr = [64, 25, 12, 22, 11];
//! selection_sort(&mut arr);
//! assert_eq!(arr, [11, 12, 22, 25, 64]);
//! ```
pub fn selection_sort<T: Ord>(slice: &mut [T]) {
    let n = slice.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            if slice[j] < slice[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            slice.swap(i, min_idx);
        }
    }
}
