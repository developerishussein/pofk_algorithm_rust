// Tests for Detect Cycle in Singly Linked List
use pofk_algorithm::linked_list_algorithms::singly_linked_list::*;
use pofk_algorithm::linked_list_algorithms::detect_cycle::has_cycle;

#[test]
fn test_has_cycle() {
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    assert!(!has_cycle(&head));
    // Cycle creation is not safe in safe Rust, so we only test acyclic case here.
}
