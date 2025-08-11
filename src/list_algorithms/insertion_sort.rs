//! 📝 Insertion Sort (Generic, In-Place)
//!
//! Sorts a mutable slice in ascending order using the insertion sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Arguments
//! * `slice` - The mutable slice to sort.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::insertion_sort::insertion_sort;
//! let mut arr = [5, 2, 4, 6, 1, 3];
//! insertion_sort(&mut arr);
//! assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
//! ```
pub fn insertion_sort<T: Ord>(slice: &mut [T]) {
    let n = slice.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}
