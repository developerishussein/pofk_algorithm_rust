// Tests for Singly Linked List Traversal
use pofk_algorithm::linked_list_algorithms::singly_linked_list::*;

#[test]
fn test_traverse() {
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    assert_eq!(traverse(&head), vec![1, 2]);
    let empty: Option<Box<ListNode<i32>>> = None;
    assert_eq!(traverse(&empty), Vec::<i32>::new());
}
