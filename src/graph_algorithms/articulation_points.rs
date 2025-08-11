//! Articulation Points (Cut Vertices, Generic, Production-Grade)
//!
//! Finds all articulation points (cut vertices) in an undirected graph.
//!
//! # Type Parameters
//! * `T`: Node type. Must implement `Eq` + `Hash` + `Clone` + `Ord`.
//!
//! # Arguments
//! * `nodes` - Slice of all nodes in the graph.
//! * `edges` - Slice of undirected edges as (from, to).
//!
//! # Returns
//! * `Vec<T>` - List of articulation points.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::graph_algorithms::articulation_points::articulation_points;
//! let nodes = [1, 2, 3, 4, 5];
//! let edges = [
//!     (1, 2), (2, 3), (3, 4), (4, 5), (3, 5)
//! ];
//! let mut points = articulation_points(&nodes, &edges);
//! points.sort();
//! assert_eq!(points, vec![2, 3]);
//! ```
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn articulation_points<T>(nodes: &[T], edges: &[(T, T)]) -> Vec<T>
where
    T: Eq + Hash + Clone + Ord,
{
    let mut adj: HashMap<T, Vec<T>> = HashMap::new();
    for (u, v) in edges {
        adj.entry(u.clone()).or_default().push(v.clone());
        adj.entry(v.clone()).or_default().push(u.clone());
    }
    let mut visited = HashSet::new();
    let mut tin = HashMap::new();
    let mut low = HashMap::new();
    let mut timer = 0;
    let mut result = HashSet::new();
    fn dfs<T: Eq + Hash + Clone + Ord>(
        u: &T,
        parent: Option<&T>,
        adj: &HashMap<T, Vec<T>>,
        visited: &mut HashSet<T>,
        tin: &mut HashMap<T, usize>,
        low: &mut HashMap<T, usize>,
        timer: &mut usize,
        result: &mut HashSet<T>,
    ) {
        visited.insert(u.clone());
        *timer += 1;
        tin.insert(u.clone(), *timer);
        low.insert(u.clone(), *timer);
        let mut children = 0;
        for v in adj.get(u).unwrap_or(&vec![]) {
            if Some(v) == parent {
                continue;
            }
            if visited.contains(v) {
                let low_u = low[u];
                let tin_v = tin[v];
                low.insert(u.clone(), low_u.min(tin_v));
            } else {
                dfs(v, Some(u), adj, visited, tin, low, timer, result);
                let low_u = low[u];
                let low_v = low[v];
                low.insert(u.clone(), low_u.min(low_v));
                if parent.is_some() && low[v] >= tin[u] {
                    result.insert(u.clone());
                }
                children += 1;
            }
        }
        if parent.is_none() && children > 1 {
            result.insert(u.clone());
        }
    }
    for node in nodes {
        if !visited.contains(node) {
            dfs(node, None, &adj, &mut visited, &mut tin, &mut low, &mut timer, &mut result);
        }
    }
    let mut out: Vec<T> = result.into_iter().collect();
    out.sort();
    out
}
