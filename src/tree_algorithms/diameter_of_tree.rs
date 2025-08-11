//! Diameter of Binary Tree (Generic, Production-Grade)
//!
//! Returns the diameter (longest path between any two nodes) of a binary tree.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithms::tree_algorithms::diameter_of_tree::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let diameter = diameter_of_tree(&root);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;

pub fn diameter_of_tree<T: Clone>(root: &Option<Box<TreeNode<T>>>) -> usize {
    fn helper<T: Clone>(node: &Option<Box<TreeNode<T>>>, max_diameter: &mut usize) -> usize {
        if let Some(n) = node {
            let left = helper(&n.left, max_diameter);
            let right = helper(&n.right, max_diameter);
            *max_diameter = (*max_diameter).max(left + right);
            1 + left.max(right)
        } else {
            0
        }
    }
    let mut max_diameter = 0;
    helper(root, &mut max_diameter);
    max_diameter
}
