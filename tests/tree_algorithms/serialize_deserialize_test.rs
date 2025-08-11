// Tests for Serialize and Deserialize Binary Tree
use pofk_algorithm::tree_algorithms::binary_tree_traversal::*;
use pofk_algorithm::tree_algorithms::serialize_deserialize::{serialize, deserialize};

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
fn test_serialize_deserialize() {
    let tree = make_test_tree();
    let data = serialize(&tree);
    let tree2 = deserialize(&data);
    assert_eq!(tree, tree2);
}
