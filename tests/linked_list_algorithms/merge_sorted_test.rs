// Tests for Merge Two Sorted Lists
use pofk_algorithms::linked_list_algorithms::singly_linked_list::*;
use pofk_algorithms::linked_list_algorithms::merge_sorted::merge_sorted;

#[test]
fn test_merge_sorted() {
    let mut l1 = Some(Box::new(ListNode::new(1)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    let mut l2 = Some(Box::new(ListNode::new(2)));
    l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    let merged = merge_sorted(l1, l2);
    assert_eq!(traverse(&merged), vec![1, 2, 3, 4]);
}
