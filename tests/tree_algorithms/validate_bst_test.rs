// Tests for Validate BST
use pofk_algorithm::tree_algorithms::binary_tree_traversal::*;
use pofk_algorithm::tree_algorithms::validate_bst::validate_bst;

fn make_bst() -> Option<Box<TreeNode<i32>>> {
    //      2
    //     / \
    //    1   3
    let mut root = Box::new(TreeNode::new(2));
    root.left = Some(Box::new(TreeNode::new(1)));
    root.right = Some(Box::new(TreeNode::new(3)));
    Some(root)
}

fn make_non_bst() -> Option<Box<TreeNode<i32>>> {
    //      5
    //     / \
    //    1   4
    //       / \
    //      3   6
    let mut root = Box::new(TreeNode::new(5));
    root.left = Some(Box::new(TreeNode::new(1)));
    let mut right = Box::new(TreeNode::new(4));
    right.left = Some(Box::new(TreeNode::new(3)));
    right.right = Some(Box::new(TreeNode::new(6)));
    root.right = Some(right);
    Some(root)
}

#[test]
fn test_validate_bst() {
    assert!(validate_bst(&make_bst()));
    assert!(!validate_bst(&make_non_bst()));
}
