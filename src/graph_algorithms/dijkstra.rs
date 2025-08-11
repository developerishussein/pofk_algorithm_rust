//! Dijkstra's Algorithm (Generic, Production-Grade)
//!
//! Computes the shortest path from a start node to all other nodes in a weighted graph.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//! * `W`: Weight type. Must implement `Copy` + `Ord` + `Default` + `Add<Output=W>`.
//!
//! # Arguments
//! * `graph` - Reference to adjacency list: `&HashMap<T, Vec<(T, W)>>`.
//! * `start` - The starting node.
//!
//! # Returns
//! * `HashMap<T, W>` - Map from node to shortest distance from start.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithm::graph_algorithms::dijkstra::dijkstra;
//! let mut graph = HashMap::new();
//! graph.insert(1, vec![(2, 1), (3, 4)]);
//! graph.insert(2, vec![(3, 2), (4, 5)]);
//! graph.insert(3, vec![(4, 1)]);
//! graph.insert(4, vec![]);
//! let dist = dijkstra(&graph, 1);
//! assert_eq!(dist[&4], 4);
//! ```
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;
use std::ops::Add;

pub fn dijkstra<T, W>(graph: &HashMap<T, Vec<(T, W)>>, start: T) -> HashMap<T, W>
where
    T: Eq + std::hash::Hash + Clone + Ord,
    W: Copy + Ord + Default + Add<Output = W>,
{
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start.clone(), W::default());
    heap.push(Reverse((W::default(), start.clone())));
    while let Some(Reverse((cost, node))) = heap.pop() {
        if let Some(neighbors) = graph.get(&node) {
            for (neighbor, weight) in neighbors {
                let next = cost + *weight;
                if dist.get(neighbor).map_or(true, |&d| next < d) {
                    dist.insert(neighbor.clone(), next);
                    heap.push(Reverse((next, neighbor.clone())));
                }
            }
        }
    }
    dist
}
