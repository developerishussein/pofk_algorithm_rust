// Tests for Insert and Delete at Position
use pofk_algorithms::linked_list_algorithms::singly_linked_list::*;
use pofk_algorithms::linked_list_algorithms::insert_delete::*;

#[test]
fn test_insert_delete() {
    let mut head = Some(Box::new(ListNode::new(1)));
    insert_at(&mut head, 1, 2);
    assert_eq!(traverse(&head), vec![1, 2]);
    insert_at(&mut head, 1, 3);
    assert_eq!(traverse(&head), vec![1, 3, 2]);
    delete_at(&mut head, 1);
    assert_eq!(traverse(&head), vec![1, 2]);
    delete_at(&mut head, 0);
    assert_eq!(traverse(&head), vec![2]);
    delete_at(&mut head, 0);
    assert_eq!(traverse(&head), Vec::<i32>::new());
}
