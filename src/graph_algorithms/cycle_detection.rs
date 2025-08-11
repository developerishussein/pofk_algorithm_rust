//! Cycle Detection (Generic, Production-Grade)
//!
//! Detects if a cycle exists in a directed graph represented as an adjacency list.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `graph` - Reference to adjacency list: `&HashMap<T, Vec<T>>`.
//!
//! # Returns
//! * `bool` - True if a cycle exists, false otherwise.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithm::graph_algorithms::cycle_detection::has_cycle;
//! let mut graph = HashMap::new();
//! graph.insert(1, vec![2]);
//! graph.insert(2, vec![3]);
//! graph.insert(3, vec![1]);
//! assert!(has_cycle(&graph));
//! graph.insert(3, vec![]);
//! assert!(!has_cycle(&graph));
//! ```
use std::collections::{HashMap, HashSet};

pub fn has_cycle<T>(graph: &HashMap<T, Vec<T>>) -> bool
where
    T: Eq + std::hash::Hash + Clone,
{
    fn visit<T: Eq + std::hash::Hash + Clone>(
        node: &T,
        graph: &HashMap<T, Vec<T>>,
        visited: &mut HashSet<T>,
        stack: &mut HashSet<T>,
    ) -> bool {
        if !visited.insert(node.clone()) {
            return stack.contains(node);
        }
        stack.insert(node.clone());
        if let Some(neighbors) = graph.get(node) {
            for neighbor in neighbors {
                if visit(neighbor, graph, visited, stack) {
                    return true;
                }
            }
        }
        stack.remove(node);
        false
    }
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    for node in graph.keys() {
        if !visited.contains(node) && visit(node, graph, &mut visited, &mut stack) {
            return true;
        }
    }
    false
}
