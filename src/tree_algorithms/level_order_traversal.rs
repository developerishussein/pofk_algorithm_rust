//! Level Order (BFS) Traversal for Binary Tree (Generic, Production-Grade)
//!
//! Returns the level order traversal (BFS) of a binary tree as a vector of vectors.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithms::tree_algorithms::level_order_traversal::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let levels = level_order_traversal(&root);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;
use std::collections::VecDeque;

pub fn level_order_traversal<T: Clone>(root: &Option<Box<TreeNode<T>>>) -> Vec<Vec<T>> {
    let mut res = Vec::new();
    if root.is_none() { return res; }
    let mut queue = VecDeque::new();
    queue.push_back(root.as_ref());
    while !queue.is_empty() {
        let mut level = Vec::new();
        for _ in 0..queue.len() {
            if let Some(Some(node)) = queue.pop_front() {
                level.push(node.val.clone());
                if node.left.is_some() { queue.push_back(node.left.as_ref()); }
                if node.right.is_some() { queue.push_back(node.right.as_ref()); }
            }
        }
        if !level.is_empty() { res.push(level); }
    }
    res
}
