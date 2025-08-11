//! Invert Binary Tree (Generic, Production-Grade)
//!
//! Inverts a binary tree (mirror image).
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithms::tree_algorithms::invert_tree::*;
//! let mut root = Some(Box::new(TreeNode::new(1)));
//! invert_tree(&mut root);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;

pub fn invert_tree<T: Clone>(root: &mut Option<Box<TreeNode<T>>>) {
    if let Some(node) = root {
        std::mem::swap(&mut node.left, &mut node.right);
        invert_tree(&mut node.left);
        invert_tree(&mut node.right);
    }
}
