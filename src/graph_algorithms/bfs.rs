//! Breadth-First Search (BFS, Generic, Production-Grade)
//!
//! Performs a breadth-first traversal on a graph represented as an adjacency list.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `graph` - Reference to adjacency list: `&HashMap<T, Vec<T>>`.
//! * `start` - The starting node.
//!
//! # Returns
//! * `Vec<T>` - The order of nodes visited in BFS.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithm::graph_algorithms::bfs::bfs;
//! let mut graph = HashMap::new();
//! graph.insert(1, vec![2, 3]);
//! graph.insert(2, vec![4]);
//! graph.insert(3, vec![]);
//! graph.insert(4, vec![]);
//! let order = bfs(&graph, 1);
//! assert_eq!(order, vec![1, 2, 3, 4]);
//! ```
use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs<T>(graph: &HashMap<T, Vec<T>>, start: T) -> Vec<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut order = Vec::new();
    queue.push_back(start.clone());
    visited.insert(start.clone());
    while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if visited.insert(neighbor.clone()) {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    order
}
