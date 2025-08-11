//! 🔗 Find Union of Two Collections (Generic, Hashable)
//!
//! Returns a Vec containing the union of two slices, with unique elements.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `a`, `b` - The slices to union.
//!
//! # Returns
//! * `Vec<T>` - The union of the two slices.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::set_algorithms::find_union::find_union;
//! let a = [1, 2, 3];
//! let b = [3, 4, 5];
//! let union = find_union(&a, &b);
//! assert_eq!(union.len(), 5);
//! ```
use std::collections::HashSet;
pub fn find_union<T: Eq + std::hash::Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let mut set = HashSet::new();
    set.extend(a.iter().cloned());
    set.extend(b.iter().cloned());
    set.into_iter().collect()
}
