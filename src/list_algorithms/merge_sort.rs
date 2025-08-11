//! 🪄 Merge Sort (Generic, Stable)
//!
//! Sorts a vector in ascending order using the merge sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord + Clone`.
//!
//! # Arguments
//! * `vec` - The vector to sort (will be sorted in-place).
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::merge_sort::merge_sort;
//! let mut arr = vec![5, 2, 4, 6, 1, 3];
//! merge_sort(&mut arr);
//! assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
//! ```
pub fn merge_sort<T: Ord + Clone>(vec: &mut Vec<T>) {
    let n = vec.len();
    if n <= 1 { return; }
    let mid = n / 2;
    let mut left = vec[..mid].to_vec();
    let mut right = vec[mid..].to_vec();
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(vec, &left, &right);
}

fn merge<T: Ord + Clone>(vec: &mut Vec<T>, left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            vec[k] = left[i].clone();
            i += 1;
        } else {
            vec[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    while i < left.len() {
        vec[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    while j < right.len() {
        vec[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
