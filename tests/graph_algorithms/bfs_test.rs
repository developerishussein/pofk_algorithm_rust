#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::graph_algorithms::bfs::bfs;

    #[test]
    fn test_bfs_basic() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2, 3]);
        graph.insert(2, vec![4]);
        graph.insert(3, vec![]);
        graph.insert(4, vec![]);
        let order = bfs(&graph, 1);
        assert_eq!(order, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_bfs_disconnected() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![]);
        graph.insert(3, vec![]);
        let order = bfs(&graph, 1);
        assert_eq!(order, vec![1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![]);
        let order = bfs(&graph, 1);
        assert_eq!(order, vec![1]);
    }
}
