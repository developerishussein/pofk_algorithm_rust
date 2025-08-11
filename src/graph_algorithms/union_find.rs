//! Union-Find (Disjoint Set, Generic, Production-Grade)
//!
//! A data structure for efficient union and find operations on disjoint sets.
//!
//! # Type Parameters
//! * `T`: Element type. Must implement `Eq` + `Hash` + `Clone`.
//!
//! # Example
//! ```rust
//! use pofk_algorithms::graph_algorithms::union_find::UnionFind;
//! let mut uf = UnionFind::new();
//! uf.add(1);
//! uf.add(2);
//! uf.add(3);
//! uf.union(1, 2);
//! assert!(uf.connected(1, 2));
//! assert!(!uf.connected(1, 3));
//! ```
use std::collections::HashMap;
use std::hash::Hash;

pub struct UnionFind<T> {
    parent: HashMap<T, T>,
    rank: HashMap<T, usize>,
}

impl<T> UnionFind<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    pub fn add(&mut self, x: T) {
        self.parent.entry(x.clone()).or_insert(x.clone());
        self.rank.entry(x).or_insert(0);
    }

    pub fn find(&mut self, x: T) -> T {
        let p = self.parent.get(&x).cloned().unwrap();
        if p != x {
            let root = self.find(p.clone());
            self.parent.insert(x.clone(), root.clone());
            root
        } else {
            x
        }
    }

    pub fn union(&mut self, x: T, y: T) {
        let x_root = self.find(x.clone());
        let y_root = self.find(y.clone());
        if x_root == y_root {
            return;
        }
        let x_rank = *self.rank.get(&x_root).unwrap();
        let y_rank = *self.rank.get(&y_root).unwrap();
        if x_rank < y_rank {
            self.parent.insert(x_root, y_root);
        } else if x_rank > y_rank {
            self.parent.insert(y_root, x_root);
        } else {
            self.parent.insert(y_root, x_root.clone());
            *self.rank.get_mut(&x_root).unwrap() += 1;
        }
    }

    pub fn connected(&mut self, x: T, y: T) -> bool {
        self.find(x) == self.find(y)
    }
}
