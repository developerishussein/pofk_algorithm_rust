//! Prim's Minimum Spanning Tree (MST, Generic, Production-Grade)
//!
//! Computes the minimum spanning tree of a weighted undirected graph using Prim's algorithm.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone` + `Ord`.
//! * `W`: Weight type. Must implement `Copy` + `Ord` + `Default`.
//!
//! # Arguments
//! * `nodes` - Slice of all nodes in the graph.
//! * `edges` - Slice of edges as (from, to, weight).
//! * `start` - The starting node for Prim's algorithm.
//!
//! # Returns
//! * `Vec<(T, T, W)>` - Edges in the MST.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::graph_algorithms::prim::prim;
//! let nodes = [1, 2, 3, 4];
//! let edges = [
//!     (1, 2, 1),
//!     (2, 3, 2),
//!     (3, 4, 1),
//!     (4, 1, 2),
//!     (1, 3, 2),
//! ];
//! let mst = prim(&nodes, &edges, 1);
//! assert_eq!(mst.len(), 3);
//! ```
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;
use std::hash::Hash;

pub fn prim<T, W>(_nodes: &[T], edges: &[(T, T, W)], start: T) -> Vec<(T, T, W)>
where
    T: Eq + Hash + Clone + Ord,
    W: Copy + Ord + Default,
{
    let mut adj: HashMap<T, Vec<(T, W)>> = HashMap::new();
    for (u, v, w) in edges {
        adj.entry(u.clone()).or_default().push((v.clone(), *w));
        adj.entry(v.clone()).or_default().push((u.clone(), *w));
    }
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    let mut mst = Vec::new();
    visited.insert(start.clone());
    for (v, w) in adj.get(&start).unwrap_or(&vec![]) {
        heap.push(Reverse((*w, start.clone(), v.clone())));
    }
    while let Some(Reverse((w, u, v))) = heap.pop() {
        if visited.contains(&v) {
            continue;
        }
        visited.insert(v.clone());
        mst.push((u.clone(), v.clone(), w));
        for (to, weight) in adj.get(&v).unwrap_or(&vec![]) {
            if !visited.contains(to) {
                heap.push(Reverse((*weight, v.clone(), to.clone())));
            }
        }
    }
    mst
}
