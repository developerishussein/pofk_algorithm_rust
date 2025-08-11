//! Zigzag Level Order Traversal for Binary Tree (Generic, Production-Grade)
//!
//! Returns the zigzag (spiral) level order traversal of a binary tree as a vector of vectors.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithms::tree_algorithms::zigzag_traversal::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let levels = zigzag_traversal(&root);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;
use std::collections::VecDeque;

pub fn zigzag_traversal<T: Clone>(root: &Option<Box<TreeNode<T>>>) -> Vec<Vec<T>> {
    let mut res = Vec::new();
    if root.is_none() { return res; }
    let mut queue = VecDeque::new();
    queue.push_back(root.as_ref());
    let mut left_to_right = true;
    while !queue.is_empty() {
        let mut level = Vec::new();
        for _ in 0..queue.len() {
            if let Some(Some(node)) = queue.pop_front() {
                if left_to_right {
                    level.push(node.val.clone());
                } else {
                    level.insert(0, node.val.clone());
                }
                if node.left.is_some() { queue.push_back(node.left.as_ref()); }
                if node.right.is_some() { queue.push_back(node.right.as_ref()); }
            }
        }
        if !level.is_empty() { res.push(level); }
        left_to_right = !left_to_right;
    }
    res
}
