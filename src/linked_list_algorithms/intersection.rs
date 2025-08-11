//! Intersection of Two Singly Linked Lists (Generic, Production-Grade)
//!
//! Finds the intersection node of two singly linked lists, if any.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone` + `Eq`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::linked_list_algorithms::intersection::*;
//! use pofk_algorithms::linked_list_algorithms::singly_linked_list::ListNode;
//! let mut a = Some(Box::new(ListNode::new(1)));
//! let mut b = Some(Box::new(ListNode::new(2)));
//! assert!(intersection(&a, &b).is_none());
//! ```
use crate::linked_list_algorithms::singly_linked_list::ListNode;

pub fn intersection<T: Clone + Eq + std::hash::Hash>(
    head_a: &Option<Box<ListNode<T>>>,
    head_b: &Option<Box<ListNode<T>>>,
) -> Option<T> {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    let mut node = head_a.as_ref();
    while let Some(n) = node {
        set.insert(n.val.clone());
        node = n.next.as_ref();
    }
    let mut node = head_b.as_ref();
    while let Some(n) = node {
        if set.contains(&n.val) {
            return Some(n.val.clone());
        }
        node = n.next.as_ref();
    }
    None
}
