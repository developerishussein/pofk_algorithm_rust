//! Singly Linked List Node and Traversal (Generic, Production-Grade)
//!
//! Provides a generic singly linked list node and a traversal function.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::linked_list_algorithms::singly_linked_list::*;
//! let mut head = Some(Box::new(ListNode::new(1)));
//! head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
//! let vals = traverse(&head);
//! assert_eq!(vals, vec![1, 2]);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListNode<T: Clone> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Clone> ListNode<T> {
    pub fn new(val: T) -> Self {
        ListNode { val, next: None }
    }
}

pub fn traverse<T: Clone>(head: &Option<Box<ListNode<T>>>) -> Vec<T> {
    let mut res = Vec::new();
    let mut node = head.as_ref();
    while let Some(n) = node {
        res.push(n.val.clone());
        node = n.next.as_ref();
    }
    res
}
