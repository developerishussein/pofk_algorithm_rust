// Tests for Binary Tree Traversals
use pofk_algorithm::tree_algorithms::binary_tree_traversal::*;

fn make_test_tree() -> Option<Box<TreeNode<i32>>> {
    //      1
    //     / \
    //    2   3
    //   / \
    //  4   5
    let mut root = Box::new(TreeNode::new(1));
    let mut left = Box::new(TreeNode::new(2));
    left.left = Some(Box::new(TreeNode::new(4)));
    left.right = Some(Box::new(TreeNode::new(5)));
    root.left = Some(left);
    root.right = Some(Box::new(TreeNode::new(3)));
    Some(root)
}

#[test]
fn test_inorder_traversal() {
    let tree = make_test_tree();
    assert_eq!(inorder_traversal(&tree), vec![4, 2, 5, 1, 3]);
}

#[test]
fn test_preorder_traversal() {
    let tree = make_test_tree();
    assert_eq!(preorder_traversal(&tree), vec![1, 2, 4, 5, 3]);
}

#[test]
fn test_postorder_traversal() {
    let tree = make_test_tree();
    assert_eq!(postorder_traversal(&tree), vec![4, 5, 2, 3, 1]);
}
