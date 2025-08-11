//! Kruskal's Minimum Spanning Tree (MST, Generic, Production-Grade)
//!
//! Computes the minimum spanning tree of a weighted undirected graph using Kruskal's algorithm.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone` + `Ord`.
//! * `W`: Weight type. Must implement `Copy` + `Ord`.
//!
//! # Arguments
//! * `nodes` - Slice of all nodes in the graph.
//! * `edges` - Slice of edges as (from, to, weight).
//!
//! # Returns
//! * `Vec<(T, T, W)>` - Edges in the MST.
//!
//! # Example
//! ```rust
//! use pofk_algorithm::graph_algorithms::kruskal::kruskal;
//! let nodes = [1, 2, 3, 4];
//! let edges = [
//!     (1, 2, 1),
//!     (2, 3, 2),
//!     (3, 4, 1),
//!     (4, 1, 2),
//!     (1, 3, 2),
//! ];
//! let mst = kruskal(&nodes, &edges);
//! assert_eq!(mst.len(), 3);
//! ```
use std::collections::HashMap;
use std::hash::Hash;

pub fn kruskal<T, W>(nodes: &[T], edges: &[(T, T, W)]) -> Vec<(T, T, W)>
where
    T: Eq + Hash + Clone + Ord,
    W: Copy + Ord,
{
    let mut parent: HashMap<T, T> = HashMap::new();
    for node in nodes {
        parent.insert(node.clone(), node.clone());
    }
    fn find<T: Eq + Hash + Clone>(parent: &mut HashMap<T, T>, x: T) -> T {
        let p = parent.get(&x).cloned().unwrap();
        if p != x {
            let root = find(parent, p.clone());
            parent.insert(x.clone(), root.clone());
            root
        } else {
            x
        }
    }
    fn union<T: Eq + Hash + Clone>(parent: &mut HashMap<T, T>, x: T, y: T) {
        let x_root = find(parent, x);
        let y_root = find(parent, y);
        if x_root != y_root {
            parent.insert(x_root, y_root);
        }
    }
    let mut sorted_edges = edges.to_vec();
    sorted_edges.sort_by_key(|e| e.2);
    let mut mst = Vec::new();
    for (u, v, w) in sorted_edges {
        if find(&mut parent, u.clone()) != find(&mut parent, v.clone()) {
            union(&mut parent, u.clone(), v.clone());
            mst.push((u, v, w));
        }
    }
    mst
}
