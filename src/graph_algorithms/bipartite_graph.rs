//! Bipartite Graph Check (Generic, Production-Grade)
//!
//! Checks if an undirected graph is bipartite using BFS coloring.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `graph` - Reference to adjacency list: `&HashMap<T, Vec<T>>`.
//!
//! # Returns
//! * `bool` - True if the graph is bipartite, false otherwise.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithm::graph_algorithms::bipartite_graph::is_bipartite;
//! let mut graph = HashMap::new();
//! graph.insert(1, vec![2, 3]);
//! graph.insert(2, vec![1, 4]);
//! graph.insert(3, vec![1, 4]);
//! graph.insert(4, vec![2, 3]);
//! assert!(is_bipartite(&graph));
//! graph.insert(4, vec![2, 3, 1]);
//! assert!(!is_bipartite(&graph));
//! ```
use std::collections::{HashMap, VecDeque};

pub fn is_bipartite<T>(graph: &HashMap<T, Vec<T>>) -> bool
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut color = HashMap::new();
    for node in graph.keys() {
        if !color.contains_key(node) {
            let mut queue = VecDeque::new();
            queue.push_back((node.clone(), true));
            while let Some((n, c)) = queue.pop_front() {
                if let Some(&col) = color.get(&n) {
                    if col != c {
                        return false;
                    }
                } else {
                    color.insert(n.clone(), c);
                    if let Some(neighbors) = graph.get(&n) {
                        for neighbor in neighbors {
                            queue.push_back((neighbor.clone(), !c));
                        }
                    }
                }
            }
        }
    }
    true
}
