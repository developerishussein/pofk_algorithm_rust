// Tests for Invert Binary Tree
use pofk_algorithm::tree_algorithms::binary_tree_traversal::*;
use pofk_algorithm::tree_algorithms::invert_tree::invert_tree;

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
fn test_invert_tree() {
    let mut tree = make_test_tree();
    invert_tree(&mut tree);
    // After invert, left and right children are swapped at every node
    // Inorder traversal should now be [3, 1, 5, 2, 4]
    use pofk_algorithm::tree_algorithms::binary_tree_traversal::inorder_traversal;
    assert_eq!(inorder_traversal(&tree), vec![3, 1, 5, 2, 4]);
}
