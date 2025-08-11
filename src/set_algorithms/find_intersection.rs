//! 🔗 Find Intersection of Two Collections (Generic, Hashable)
//!
//! Returns a Vec containing the intersection of two slices.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `a`, `b` - The slices to intersect.
//!
//! # Returns
//! * `Vec<T>` - The intersection of the two slices.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::find_intersection::find_intersection;
//! let a = [1, 2, 3];
//! let b = [2, 3, 4];
//! let mut inter = find_intersection(&a, &b);
//! inter.sort();
//! let mut expected = vec![2, 3];
//! expected.sort();
//! assert_eq!(inter, expected);
//! ```
use std::collections::HashSet;
pub fn find_intersection<T: Eq + std::hash::Hash + Clone>(a: &[T], b: &[T]) -> Vec<T> {
    let set_a: HashSet<_> = a.iter().collect();
    let set_b: HashSet<_> = b.iter().collect();
    set_a.intersection(&set_b).map(|&x| x.clone()).collect()
}
