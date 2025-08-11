//! ⚡ Quick Sort (Generic, In-Place)
//!
//! Sorts a mutable slice in ascending order using the quick sort algorithm.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Ord`.
//!
//! # Arguments
//! * `slice` - The mutable slice to sort.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::list_algorithms::quick_sort::quick_sort;
//! let mut arr = [5, 2, 4, 6, 1, 3];
//! quick_sort(&mut arr);
//! assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
//! ```
pub fn quick_sort<T: Ord>(slice: &mut [T]) {
    let len = slice.len();
    if len <= 1 { return; }
    _quick_sort(slice, 0, len - 1);
}

fn _quick_sort<T: Ord>(slice: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition(slice, low, high);
        if p > 0 { _quick_sort(slice, low, p - 1); }
        _quick_sort(slice, p + 1, high);
    }
}

fn partition<T: Ord>(slice: &mut [T], low: usize, high: usize) -> usize {
    let pivot = high;
    let mut i = low;
    for j in low..high {
        if slice[j] <= slice[pivot] {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, pivot);
    i
}
