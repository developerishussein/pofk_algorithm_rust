//! 🔄 Rotate Array Right (Generic, In-Place)
//!
//! Rotates a mutable slice to the right by `k` steps.
//!
//! # Type Parameters
//! * `T`: The element type.
//!
//! # Arguments
//! * `slice` - The mutable slice to rotate.
//! * `k` - The number of steps to rotate.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::list_algorithms::rotate_array_right::rotate_array_right;
//! let mut arr = [1, 2, 3, 4, 5, 6, 7];
//! rotate_array_right(&mut arr, 3);
//! assert_eq!(arr, [5, 6, 7, 1, 2, 3, 4]);
//! ```
pub fn rotate_array_right<T>(slice: &mut [T], k: usize) {
    let n = slice.len();
    if n == 0 { return; }
    let k = k % n;
    slice.reverse();
    slice[..k].reverse();
    slice[k..].reverse();
}
