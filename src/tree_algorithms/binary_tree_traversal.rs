//! Binary Tree Traversals: Inorder, Preorder, Postorder (Generic, Production-Grade)
//!
//! Provides generic implementations for inorder, preorder, and postorder traversals of a binary tree.
//!
//! # Type Parameters
//! * `T`: Node value type. Must implement `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::tree_algorithms::binary_tree_traversal::*;
//! let root = Some(Box::new(TreeNode::new(1)));
//! let inorder = inorder_traversal(&root);
//! let preorder = preorder_traversal(&root);
//! let postorder = postorder_traversal(&root);
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode<T: Clone> {
    pub val: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}

impl<T: Clone> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

pub fn inorder_traversal<T: Clone>(root: &Option<Box<TreeNode<T>>>) -> Vec<T> {
    let mut res = Vec::new();
    fn helper<T: Clone>(node: &Option<Box<TreeNode<T>>>, res: &mut Vec<T>) {
        if let Some(n) = node {
            helper(&n.left, res);
            res.push(n.val.clone());
            helper(&n.right, res);
        }
    }
    helper(root, &mut res);
    res
}

pub fn preorder_traversal<T: Clone>(root: &Option<Box<TreeNode<T>>>) -> Vec<T> {
    let mut res = Vec::new();
    fn helper<T: Clone>(node: &Option<Box<TreeNode<T>>>, res: &mut Vec<T>) {
        if let Some(n) = node {
            res.push(n.val.clone());
            helper(&n.left, res);
            helper(&n.right, res);
        }
    }
    helper(root, &mut res);
    res
}

pub fn postorder_traversal<T: Clone>(root: &Option<Box<TreeNode<T>>>) -> Vec<T> {
    let mut res = Vec::new();
    fn helper<T: Clone>(node: &Option<Box<TreeNode<T>>>, res: &mut Vec<T>) {
        if let Some(n) = node {
            helper(&n.left, res);
            helper(&n.right, res);
            res.push(n.val.clone());
        }
    }
    helper(root, &mut res);
    res
}
