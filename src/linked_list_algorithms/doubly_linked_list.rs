//! Doubly Linked List Node and Reverse (Generic, Production-Grade)
//!
//! Provides a generic doubly linked list node and a reverse function.
//!
//! Note: For safety and simplicity, the `prev` pointers in the reversed list are set to `None`.
//! A fully functional doubly linked list with correct `prev` pointers requires a more complex approach (e.g., using Rc/RefCell).
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::linked_list_algorithms::doubly_linked_list::*;
//! let mut head = Some(Box::new(DListNode::new(1)));
//! head.as_mut().unwrap().next = Some(Box::new(DListNode::new(2)));
//! head.as_mut().unwrap().next.as_mut().unwrap().prev = Some(head.as_mut().unwrap().as_mut() as *mut _);
//! let rev = reverse_doubly(&head);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DListNode<T: Clone> {
    pub val: T,
    pub prev: Option<*mut DListNode<T>>,
    pub next: Option<Box<DListNode<T>>>,
}

impl<T: Clone> DListNode<T> {
    pub fn new(val: T) -> Self {
        DListNode { val, prev: None, next: None }
    }
}

pub fn reverse_doubly<T: Clone>(head: &Option<Box<DListNode<T>>>) -> Option<Box<DListNode<T>>> {
    let mut curr = head.as_ref();
    let mut prev: Option<Box<DListNode<T>>> = None;
    while let Some(node) = curr {
        let mut new_node = Box::new(DListNode::new(node.val.clone()));
        new_node.next = prev;
        // For safety, we do not set prev pointers in the reversed list.
        prev = Some(new_node);
        curr = node.next.as_ref();
    }
    prev
}
