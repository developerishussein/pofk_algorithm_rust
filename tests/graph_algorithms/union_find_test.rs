#[cfg(test)]
mod tests {
    use crate::graph_algorithms::union_find::UnionFind;

    #[test]
    fn test_union_find_basic() {
        let mut uf = UnionFind::new();
        uf.add(1);
        uf.add(2);
        uf.add(3);
        uf.union(1, 2);
        assert!(uf.connected(1, 2));
        assert!(!uf.connected(1, 3));
    }

    #[test]
    fn test_union_find_chain() {
        let mut uf = UnionFind::new();
        for i in 1..=5 {
            uf.add(i);
        }
        uf.union(1, 2);
        uf.union(2, 3);
        uf.union(3, 4);
        assert!(uf.connected(1, 4));
        assert!(!uf.connected(1, 5));
    }
}
