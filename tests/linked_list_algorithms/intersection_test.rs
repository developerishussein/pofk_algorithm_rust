// Tests for Intersection of Two Lists
use pofk_algorithm::linked_list_algorithms::singly_linked_list::*;
use pofk_algorithm::linked_list_algorithms::intersection::intersection;

#[test]
fn test_intersection() {
    let mut a = Some(Box::new(ListNode::new(1)));
    a.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    let mut b = Some(Box::new(ListNode::new(3)));
    b.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    assert_eq!(intersection(&a, &b), Some(2));
    let mut c = Some(Box::new(ListNode::new(4)));
    assert_eq!(intersection(&a, &c), None);
}
