//! 🫧 Bubble Sort (Generic, In-Place)
//!
//! Sorts a mutable slice in ascending order using the bubble sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Arguments
//! * `slice` - The mutable slice to sort.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::list_algorithms::bubble_sort::bubble_sort;
//! let mut arr = [5, 2, 4, 6, 1, 3];
//! bubble_sort(&mut arr);
//! assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
//! ```
pub fn bubble_sort<T: Ord>(slice: &mut [T]) {
    let n = slice.len();
    if n < 2 { return; }
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped { break; }
    }
}
