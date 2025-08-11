//! ➖ Set Difference (Generic, Hashable)
//!
//! Returns a Vec containing elements in `a` that are not in `b`.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `a`, `b` - The slices to compute the difference.
//!
//! # Returns
//! * `Vec<T>` - Elements in `a` not in `b`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::set_algorithms::set_difference::set_difference;
//! let a = [1, 2, 3, 4];
//! let b = [2, 4];
//! let diff = set_difference(&a, &b);
//! assert_eq!(diff, vec![1, 3]);
//! ```
use std::collections::HashSet;
pub fn set_difference<T: Eq + std::hash::Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let set_b: HashSet<_> = b.iter().collect();
    a.iter().filter(|x| !set_b.contains(x)).cloned().collect()
}
