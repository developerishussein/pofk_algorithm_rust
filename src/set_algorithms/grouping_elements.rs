//! Grouping Elements (Generic, Hashable, Keyed)
//!
//! Groups elements of a slice by a key function, returning a map from key to vector of elements.
//!
//! # Type Parameters
//! * `T`: The element type. Must implement `Clone`.
//! * `K`: The key type. Must implement `Eq` + `Hash`.
//!
//! # Arguments
//! * `slice` - The slice to group.
//! * `key_fn` - Function to extract the key from each element.
//!
//! # Returns
//! * `HashMap<K, Vec<T>>` - A map from key to vectors of elements.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::set_algorithms::grouping_elements::grouping_elements;
//! let arr = ["apple", "apricot", "banana", "blueberry"];
//! let groups = grouping_elements(&arr, |s| s.chars().next().unwrap());
//! assert_eq!(groups[&'a'], vec!["apple", "apricot"]);
//! assert_eq!(groups[&'b'], vec!["banana", "blueberry"]);
//! ```
use std::collections::HashMap;

pub fn grouping_elements<T, K, F>(slice: &[T], mut key_fn: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Eq + std::hash::Hash,
    F: FnMut(&T) -> K,
{
    let mut groups: HashMap<K, Vec<T>> = HashMap::new();
    for item in slice {
        groups.entry(key_fn(item)).or_default().push(item.clone());
    }
    groups
}
