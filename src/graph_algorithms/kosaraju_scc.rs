//! Kosaraju's Strongly Connected Components (SCC, Generic, Production-Grade)
//!
//! Finds all strongly connected components in a directed graph using Kosaraju's algorithm.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone` + `Ord`.
//!
//! # Arguments
//! * `nodes` - Slice of all nodes in the graph.
//! * `edges` - Slice of directed edges as (from, to).
//!
//! # Returns
//! * `Vec<Vec<T>>` - List of strongly connected components, each as a vector of nodes.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::graph_algorithms::kosaraju_scc::kosaraju_scc;
//! let nodes = [1, 2, 3, 4, 5];
//! let edges = [
//!     (1, 2), (2, 3), (3, 1),
//!     (3, 4), (4, 5), (5, 4)
//! ];
//! let mut sccs = kosaraju_scc(&nodes, &edges);
//! for scc in &mut sccs { scc.sort(); }
//! sccs.sort();
//! assert_eq!(sccs, vec![vec![1,2,3], vec![4,5]]);
//! ```
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn kosaraju_scc<T>(nodes: &[T], edges: &[(T, T)]) -> Vec<Vec<T>>
where
    T: Eq + Hash + Clone + Ord,
{
    let mut adj: HashMap<T, Vec<T>> = HashMap::new();
    let mut radj: HashMap<T, Vec<T>> = HashMap::new();
    for (u, v) in edges {
        adj.entry(u.clone()).or_default().push(v.clone());
        radj.entry(v.clone()).or_default().push(u.clone());
    }
    let mut visited = HashSet::new();
    let mut order = Vec::new();
    fn dfs<T: Eq + Hash + Clone>(u: &T, adj: &HashMap<T, Vec<T>>, visited: &mut HashSet<T>, order: &mut Vec<T>) {
        if !visited.insert(u.clone()) { return; }
        for v in adj.get(u).unwrap_or(&vec![]) {
            dfs(v, adj, visited, order);
        }
        order.push(u.clone());
    }
    for node in nodes {
        dfs(node, &adj, &mut visited, &mut order);
    }
    let mut visited = HashSet::new();
    let mut sccs = Vec::new();
    fn rdfs<T: Eq + Hash + Clone>(u: &T, radj: &HashMap<T, Vec<T>>, visited: &mut HashSet<T>, comp: &mut Vec<T>) {
        if !visited.insert(u.clone()) { return; }
        comp.push(u.clone());
        for v in radj.get(u).unwrap_or(&vec![]) {
            rdfs(v, radj, visited, comp);
        }
    }
    for u in order.iter().rev() {
        if !visited.contains(u) {
            let mut comp = Vec::new();
            rdfs(u, &radj, &mut visited, &mut comp);
            sccs.push(comp);
        }
    }
    sccs
}
