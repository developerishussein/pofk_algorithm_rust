//! Lowest Common Ancestor (LCA) in Binary Tree (Generic, Production-Grade)
//!
//! Finds the lowest common ancestor of two nodes in a binary tree.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone` + `Eq`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::tree_algorithms::binary_tree_traversal::TreeNode;
//! use pofk_algorithms::tree_algorithms::lowest_common_ancestor::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let lca = lowest_common_ancestor(&root, &2, &3);
//! ```
use crate::tree_algorithms::binary_tree_traversal::TreeNode;

pub fn lowest_common_ancestor<T: Clone + Eq>(
    root: &Option<Box<TreeNode<T>>>,
    p: &T,
    q: &T,
) -> Option<T> {
    fn helper<T: Clone + Eq>(
        node: &Option<Box<TreeNode<T>>>,
        p: &T,
        q: &T,
    ) -> Option<T> {
        if let Some(n) = node {
            if &n.val == p || &n.val == q {
                return Some(n.val.clone());
            }
            let left = helper(&n.left, p, q);
            let right = helper(&n.right, p, q);
            match (left, right) {
                (Some(_l), Some(_r)) => Some(n.val.clone()),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        } else {
            None
        }
    }
    helper(root, p, q)
}
