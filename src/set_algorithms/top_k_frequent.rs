//! Top K Frequent Elements (Generic, Hashable)
//!
//! Returns the k most frequent elements in the slice, in arbitrary order.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `slice` - The slice to search.
//! * `k` - The number of top elements to return.
//!
//! # Returns
//! * `Vec<T>` - The k most frequent elements.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::set_algorithms::top_k_frequent::top_k_frequent;
//! let arr = [1, 1, 1, 2, 2, 3];
//! let mut top = top_k_frequent(&arr, 2);
//! top.sort();
//! assert_eq!(top, vec![1, 2]);
//! ```
use std::collections::HashMap;

pub fn top_k_frequent<T: Eq + std::hash::Hash + Clone>(slice: &[T], k: usize) -> Vec<T> {
    let mut freq = HashMap::with_capacity(slice.len());
    for item in slice {
        *freq.entry(item.clone()).or_insert(0) += 1;
    }
    let mut freq_vec: Vec<_> = freq.into_iter().collect();
    freq_vec.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    freq_vec.into_iter().take(k).map(|(item, _)| item).collect()
}
