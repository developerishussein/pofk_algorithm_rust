//! Floyd-Warshall Algorithm (Generic, Production-Grade)
//!
//! Computes shortest paths between all pairs of nodes in a weighted graph.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone`.
//! * `W`: Weight type. Must implement `Copy` + `Ord` + `Default` + `Add<Output=W>` + `PartialOrd`.
//!
//! # Arguments
//! * `nodes` - Slice of all nodes in the graph.
//! * `edges` - Slice of edges as (from, to, weight).
//!
//! # Returns
//! * `HashMap<(T, T), W>` - Map from (source, target) to shortest distance.
//!
//! # Example
//! ```rust
//! use std::collections::HashMap;
//! use pofk_algorithm::graph_algorithms::floyd_warshall::floyd_warshall;
//! let nodes = [1, 2, 3, 4];
//! let edges = [
//!     (1, 2, 1),
//!     (2, 3, 2),
//!     (1, 3, 4),
//!     (3, 4, 1),
//! ];
//! let dist = floyd_warshall(&nodes, &edges);
//! assert_eq!(dist[&(1, 4)], 0);
//! ```
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Add;

pub fn floyd_warshall<T, W>(nodes: &[T], edges: &[(T, T, W)]) -> HashMap<(T, T), W>
where
    T: Eq + Hash + Clone,
    W: Copy + Ord + Default + Add<Output = W> + PartialOrd,
{
    let mut dist = HashMap::new();
    let inf = W::default() + W::default() + W::default() + W::default() + W::default();
    for u in nodes {
        for v in nodes {
            if u == v {
                dist.insert((u.clone(), v.clone()), W::default());
            } else {
                dist.insert((u.clone(), v.clone()), inf);
            }
        }
    }
    for (u, v, w) in edges {
        dist.insert((u.clone(), v.clone()), *w);
    }
    for k in nodes {
        for i in nodes {
            for j in nodes {
                let ij = dist[&(i.clone(), j.clone())];
                let ik = dist[&(i.clone(), k.clone())];
                let kj = dist[&(k.clone(), j.clone())];
                if ik + kj < ij {
                    dist.insert((i.clone(), j.clone()), ik + kj);
                }
            }
        }
    }
    dist
}
