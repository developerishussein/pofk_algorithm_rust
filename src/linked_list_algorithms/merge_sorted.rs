//! Merge Two Sorted Singly Linked Lists (Generic, Production-Grade)
//!
//! Merges two sorted singly linked lists into one sorted list.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone` + `Ord`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::linked_list_algorithms::merge_sorted::*;
//! use pofk_algorithm::linked_list_algorithms::singly_linked_list::ListNode;
//! let l1 = Some(Box::new(ListNode::new(1)));
//! let l2 = Some(Box::new(ListNode::new(2)));
//! let merged = merge_sorted(l1, l2);
//! ```
use crate::linked_list_algorithms::singly_linked_list::ListNode;

pub fn merge_sorted<T: Clone + Ord + Default>(
    l1: Option<Box<ListNode<T>>>,
    l2: Option<Box<ListNode<T>>>,
) -> Option<Box<ListNode<T>>> {
    let mut dummy = Box::new(ListNode::new(Default::default()));
    let mut tail = &mut dummy;
    let (mut l1, mut l2) = (l1, l2);
    while l1.is_some() && l2.is_some() {
        let (v1, v2) = (l1.as_ref().unwrap().val.clone(), l2.as_ref().unwrap().val.clone());
        if v1 <= v2 {
            let next = l1.as_mut().unwrap().next.take();
            tail.next = l1;
            tail = tail.next.as_mut().unwrap();
            l1 = next;
        } else {
            let next = l2.as_mut().unwrap().next.take();
            tail.next = l2;
            tail = tail.next.as_mut().unwrap();
            l2 = next;
        }
    }
    tail.next = if l1.is_some() { l1 } else { l2 };
    dummy.next
}
