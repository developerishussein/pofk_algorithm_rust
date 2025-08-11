#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::graph_algorithms::bipartite_graph::is_bipartite;

    #[test]
    fn test_bipartite_true() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1, 4]);
        graph.insert(3, vec![1, 4]);
        graph.insert(4, vec![2, 3]);
        assert!(is_bipartite(&graph));
    }

    #[test]
    fn test_bipartite_false() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![1, 4]);
        graph.insert(3, vec![1, 4]);
        graph.insert(4, vec![2, 3, 1]);
        assert!(!is_bipartite(&graph));
    }
}
