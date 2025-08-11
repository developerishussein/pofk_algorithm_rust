//! Balanced Binary Tree Check (Generic, Production-Grade)
//!
//! Checks if a binary tree is height-balanced.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithms::tree_algorithms::balanced_tree::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let balanced = is_balanced(&root);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;

pub fn is_balanced<T: Clone>(root: &Option<Box<TreeNode<T>>>) -> bool {
    fn helper<T: Clone>(node: &Option<Box<TreeNode<T>>>) -> Option<usize> {
        match node {
            Some(n) => {
                let l = helper(&n.left)?;
                let r = helper(&n.right)?;
                if l > r + 1 || r > l + 1 {
                    None
                } else {
                    Some(1 + l.max(r))
                }
            }
            None => Some(0),
        }
    }
    helper(root).is_some()
}
