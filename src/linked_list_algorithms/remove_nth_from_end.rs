//! Remove Nth Node From End of Singly Linked List (Generic, Production-Grade)
//!
//! Removes the nth node from the end of a singly linked list.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::linked_list_algorithms::remove_nth_from_end::*;
//! use pofk_algorithm::linked_list_algorithms::singly_linked_list::ListNode;
//! let mut head = Some(Box::new(ListNode::new(1)));
//! head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
//! let head = remove_nth_from_end(head, 1);
//! ```
use crate::linked_list_algorithms::singly_linked_list::ListNode;

pub fn remove_nth_from_end<T: Clone + Default>(head: Option<Box<ListNode<T>>>, n: usize) -> Option<Box<ListNode<T>>> {
    let mut dummy = Box::new(ListNode::new(Default::default()));
    dummy.next = head;
    let mut fast: *mut ListNode<T> = &mut *dummy;
    let mut slow: *mut ListNode<T> = &mut *dummy;
    unsafe {
        for _ in 0..=n {
            if let Some(ref mut next) = (*fast).next {
                fast = &mut **next as *mut _;
            } else {
                return dummy.next;
            }
        }
        while let Some(ref mut next) = (*fast).next {
            fast = &mut **next as *mut _;
            if let Some(ref mut next2) = (*slow).next {
                slow = &mut **next2 as *mut _;
            }
        }
        let to_remove = (*slow).next.as_mut().map(|n| n.next.take());
        (*slow).next = to_remove.unwrap_or(None);
    }
    dummy.next
}
