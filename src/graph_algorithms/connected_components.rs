//! Connected Components (Generic, Production-Grade)
//!
//! Finds all connected components in an undirected graph represented as an adjacency list.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Arguments
//! * `graph` - Reference to adjacency list: `&HashMap<T, Vec<T>>`.
//!
//! # Returns
//! * `Vec<Vec<T>>` - List of connected components, each as a vector of nodes.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithms::graph_algorithms::connected_components::connected_components;
//! let mut graph = HashMap::new();
//! graph.insert(1, vec![2]);
//! graph.insert(2, vec![1]);
//! graph.insert(3, vec![4]);
//! graph.insert(4, vec![3]);
//! let mut comps = connected_components(&graph);
//! for comp in &mut comps { comp.sort(); }
//! comps.sort();
//! assert_eq!(comps, vec![vec![1, 2], vec![3, 4]]);
//! ```
use std::collections::{HashMap, HashSet, VecDeque};

pub fn connected_components<T>(graph: &HashMap<T, Vec<T>>) -> Vec<Vec<T>>
where
    T: Eq + std::hash::Hash + Clone,
{
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    for node in graph.keys() {
        if !visited.contains(node) {
            let mut queue = VecDeque::new();
            let mut comp = Vec::new();
            queue.push_back(node.clone());
            visited.insert(node.clone());
            while let Some(n) = queue.pop_front() {
                comp.push(n.clone());
                if let Some(neighbors) = graph.get(&n) {
                    for neighbor in neighbors {
                        if visited.insert(neighbor.clone()) {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
            components.push(comp);
        }
    }
    components
}
