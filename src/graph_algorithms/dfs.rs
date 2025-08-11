//! Depth-First Search (DFS, Generic, Production-Grade)
//!
//! Performs a depth-first traversal on a graph represented as an adjacency list.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `graph` - Reference to adjacency list: `&HashMap<T, Vec<T>>`.
//! * `start` - The starting node.
//!
//! # Returns
//! * `Vec<T>` - The order of nodes visited in DFS.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithms::graph_algorithms::dfs::dfs;
//! let mut graph = HashMap::new();
//! graph.insert(1, vec![2, 3]);
//! graph.insert(2, vec![4]);
//! graph.insert(3, vec![]);
//! graph.insert(4, vec![]);
//! let order = dfs(&graph, 1);
//! assert_eq!(order, vec![1, 2, 4, 3]);
//! ```
use std::collections::{HashMap, HashSet};

pub fn dfs<T>(graph: &HashMap<T, Vec<T>>, start: T) -> Vec<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut visited = HashSet::new();
    let mut stack = vec![start.clone()];
    let mut order = Vec::new();
    while let Some(node) = stack.pop() {
        if visited.insert(node.clone()) {
            order.push(node.clone());
            if let Some(neighbors) = graph.get(&node) {
                for neighbor in neighbors.iter().rev() {
                    stack.push(neighbor.clone());
                }
            }
        }
    }
    order
}
