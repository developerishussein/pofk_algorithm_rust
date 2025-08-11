// Tests for Level Order (BFS) Traversal
use pofk_algorithm::tree_algorithms::binary_tree_traversal::*;
use pofk_algorithm::tree_algorithms::level_order_traversal::level_order_traversal;

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
fn test_level_order_traversal() {
    let tree = make_test_tree();
    let levels = level_order_traversal(&tree);
    assert_eq!(levels, vec![vec![1], vec![2, 3], vec![4, 5]]);
}
