//! Insert and Delete at Position in Singly Linked List (Generic, Production-Grade)
//!
//! Provides functions to insert and delete nodes at a given position in a singly linked list.
//!
//! # Type Parameters
//! * `T`: Value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::linked_list_algorithms::insert_delete::*;
//! use pofk_algorithm::linked_list_algorithms::singly_linked_list::{ListNode, traverse};
//! let mut head = Some(Box::new(ListNode::new(1)));
//! insert_at(&mut head, 1, 2);
//! assert_eq!(traverse(&head), vec![1, 2]);
//! delete_at(&mut head, 0);
//! assert_eq!(traverse(&head), vec![2]);
//! ```
use crate::linked_list_algorithms::singly_linked_list::ListNode;

pub fn insert_at<T: Clone>(head: &mut Option<Box<ListNode<T>>>, pos: usize, val: T) {
    let mut dummy = Box::new(ListNode::new(val.clone()));
    dummy.next = head.take();
    let mut curr = &mut dummy;
    for _ in 0..pos {
        if curr.next.is_none() { break; }
        curr = curr.next.as_mut().unwrap();
    }
    let mut node = Box::new(ListNode::new(val));
    node.next = curr.next.take();
    curr.next = Some(node);
    *head = dummy.next;
}

pub fn delete_at<T: Clone + Default>(head: &mut Option<Box<ListNode<T>>>, pos: usize) {
    let mut dummy = Box::new(ListNode::new(Default::default()));
    dummy.next = head.take();
    let mut curr = &mut dummy;
    for _ in 0..pos {
        if curr.next.is_none() { break; }
        curr = curr.next.as_mut().unwrap();
    }
    if curr.next.is_some() {
        curr.next = curr.next.as_mut().unwrap().next.take();
    }
    *head = dummy.next;
}
