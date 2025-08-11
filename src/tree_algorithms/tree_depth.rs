//! Depth of Binary Tree (Generic, Production-Grade)
//!
//! Returns the maximum depth of a binary tree.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithms::tree_algorithms::tree_depth::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let depth = tree_depth(&root);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;

pub fn tree_depth<T: Clone>(root: &Option<Box<TreeNode<T>>>) -> usize {
    match root {
        Some(node) => 1 + usize::max(tree_depth(&node.left), tree_depth(&node.right)),
        None => 0,
    }
}
