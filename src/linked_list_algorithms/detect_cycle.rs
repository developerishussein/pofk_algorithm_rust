//! Detect Cycle in Singly Linked List (Floyd's Tortoise and Hare, Generic, Production-Grade)
//!
//! Detects if a singly linked list has a cycle.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::linked_list_algorithms::detect_cycle::*;
//! use pofk_algorithms::linked_list_algorithms::singly_linked_list::ListNode;
//! let mut head = Some(Box::new(ListNode::new(1)));
//! head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
//! assert!(!has_cycle(&head));
//! ```
use crate::linked_list_algorithms::singly_linked_list::ListNode;

pub fn has_cycle<T: Clone>(head: &Option<Box<ListNode<T>>>) -> bool {
    let mut slow = head.as_ref().map(|n| &**n);
    let mut fast = head.as_ref().map(|n| &**n);
    while let (Some(s), Some(f)) = (slow, fast) {
        slow = s.next.as_ref().map(|n| &**n);
        fast = f.next.as_ref().and_then(|n| n.next.as_ref().map(|nn| &**nn));
        if let (Some(s_ptr), Some(f_ptr)) = (slow, fast) {
            if std::ptr::eq(s_ptr, f_ptr) {
                return true;
            }
        }
    }
    false
}
