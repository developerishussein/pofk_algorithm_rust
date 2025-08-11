// Tests for Balanced Binary Tree
use pofk_algorithm::tree_algorithms::binary_tree_traversal::*;
use pofk_algorithm::tree_algorithms::balanced_tree::is_balanced;

fn make_balanced_tree() -> Option<Box<TreeNode<i32>>> {
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

fn make_unbalanced_tree() -> Option<Box<TreeNode<i32>>> {
    //      1
    //     /
    //    2
    //   /
    //  3
    let mut root = Box::new(TreeNode::new(1));
    let mut left = Box::new(TreeNode::new(2));
    left.left = Some(Box::new(TreeNode::new(3)));
    root.left = Some(left);
    Some(root)
}

#[test]
fn test_is_balanced() {
    assert!(is_balanced(&make_balanced_tree()));
    assert!(!is_balanced(&make_unbalanced_tree()));
}
