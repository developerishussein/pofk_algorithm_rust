//! Validate Binary Search Tree (BST, Generic, Production-Grade)
//!
//! Checks if a binary tree is a valid BST.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone` + `Ord`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithm::tree_algorithms::validate_bst::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let valid = validate_bst(&root);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;

pub fn validate_bst<T: Clone + Ord>(root: &Option<Box<TreeNode<T>>>) -> bool {
    fn helper<T: Clone + Ord>(
        node: &Option<Box<TreeNode<T>>>,
        min: Option<&T>,
        max: Option<&T>,
    ) -> bool {
        if let Some(n) = node {
            if min.map_or(false, |v| n.val <= *v) || max.map_or(false, |v| n.val >= *v) {
                return false;
            }
            helper(&n.left, min, Some(&n.val)) && helper(&n.right, Some(&n.val), max)
        } else {
            true
        }
    }
    helper(root, None, None)
}
