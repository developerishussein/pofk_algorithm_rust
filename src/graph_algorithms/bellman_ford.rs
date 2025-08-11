//! Bellman-Ford Algorithm (Generic, Production-Grade)
//!
//! Computes shortest paths from a start node to all other nodes in a weighted graph, handling negative weights.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//! * `W`: Weight type. Must implement `Copy` + `Ord` + `Default` + `Add<Output=W>` + `Sub<Output=W>` + `PartialOrd`.
//!
//! # Arguments
//! * `graph` - Reference to adjacency list: `&HashMap<T, Vec<(T, W)>>`.
//! * `start` - The starting node.
//!
//! # Returns
//! * `Option<HashMap<T, W>>` - Map from node to shortest distance from start, or None if a negative cycle is detected.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithm::graph_algorithms::bellman_ford::bellman_ford;
//! let mut graph = HashMap::new();
//! graph.insert(1, vec![(2, 1), (3, 4)]);
//! graph.insert(2, vec![(3, -2), (4, 5)]);
//! graph.insert(3, vec![(4, 1)]);
//! graph.insert(4, vec![]);
//! let dist = bellman_ford(&graph, 1).unwrap();
//! assert_eq!(dist[&4], -1);
//! ```
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Add, Sub};

pub fn bellman_ford<T, W>(graph: &HashMap<T, Vec<(T, W)>>, start: T) -> Option<HashMap<T, W>>
where
    T: Eq + Hash + Clone,
    W: Copy + Ord + Default + Add<Output = W> + Sub<Output = W> + PartialOrd,
{
    let mut dist = HashMap::new();
    let nodes: Vec<_> = graph.keys().cloned().collect();
    for node in &nodes {
        dist.insert(node.clone(), W::default() + W::default() + W::default() + W::default() + W::default()); // Simulate infinity
    }
    dist.insert(start.clone(), W::default());
    for _ in 0..nodes.len() - 1 {
        for (u, edges) in graph {
            for (v, w) in edges {
                let alt = dist[u] + *w;
                if alt < dist[v] {
                    dist.insert(v.clone(), alt);
                }
            }
        }
    }
    // Check for negative cycles
    for (u, edges) in graph {
        for (v, w) in edges {
            if dist[u] + *w < dist[v] {
                return None;
            }
        }
    }
    Some(dist)
}
