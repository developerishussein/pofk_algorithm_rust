//! Topological Sort (Kahn's Algorithm, Generic, Production-Grade)
//!
//! Returns a topological ordering of a directed acyclic graph (DAG) represented as an adjacency list.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `graph` - Reference to adjacency list: `&HashMap<T, Vec<T>>`.
//!
//! # Returns
//! * `Option<Vec<T>>` - Topological order if DAG, None if cycle detected.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithm::graph_algorithms::topological_sort::topological_sort;
//! let mut graph = HashMap::new();
//! graph.insert(5, vec![2, 0]);
//! graph.insert(4, vec![0, 1]);
//! graph.insert(2, vec![3]);
//! graph.insert(3, vec![1]);
//! graph.insert(0, vec![]);
//! graph.insert(1, vec![]);
//! let order = topological_sort(&graph).unwrap();
//! assert_eq!(order.len(), 6);
//! ```
use std::collections::{HashMap, VecDeque};

pub fn topological_sort<T>(graph: &HashMap<T, Vec<T>>) -> Option<Vec<T>>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut in_degree = HashMap::new();
    for node in graph.keys() {
        in_degree.entry(node.clone()).or_insert(0);
    }
    for neighbors in graph.values() {
        for neighbor in neighbors {
            *in_degree.entry(neighbor.clone()).or_insert(0) += 1;
        }
    }
    let mut queue = VecDeque::new();
    for (node, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(node.clone());
        }
    }
    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                let deg = in_degree.get_mut(neighbor).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    if order.len() == in_degree.len() { Some(order) } else { None }
}
