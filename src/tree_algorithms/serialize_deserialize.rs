//! Serialize and Deserialize Binary Tree (Generic, Production-Grade)
//!
//! Provides functions to serialize a binary tree to a vector and deserialize it back.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone` + `Default` + `PartialEq`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithm::tree_algorithms::serialize_deserialize::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let data = serialize(&root);
//! let tree = deserialize(&data);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;

pub fn serialize<T: Clone + Default + PartialEq>(root: &Option<Box<TreeNode<T>>>) -> Vec<Option<T>> {
    let mut res = Vec::new();
    fn helper<T: Clone + Default + PartialEq>(node: &Option<Box<TreeNode<T>>>, res: &mut Vec<Option<T>>) {
        if let Some(n) = node {
            res.push(Some(n.val.clone()));
            helper(&n.left, res);
            helper(&n.right, res);
        } else {
            res.push(None);
        }
    }
    helper(root, &mut res);
    res
}

pub fn deserialize<T: Clone + Default + PartialEq>(data: &[Option<T>]) -> Option<Box<TreeNode<T>>> {
    fn helper<T: Clone + Default + PartialEq>(data: &[Option<T>], idx: &mut usize) -> Option<Box<TreeNode<T>>> {
        if *idx >= data.len() { return None; }
        if let Some(val) = &data[*idx] {
            let mut node = Box::new(TreeNode::new(val.clone()));
            *idx += 1;
            node.left = helper(data, idx);
            *idx += 1;
            node.right = helper(data, idx);
            Some(node)
        } else {
            None
        }
    }
    if data.is_empty() { return None; }
    let mut idx = 0;
    helper(data, &mut idx)
}
