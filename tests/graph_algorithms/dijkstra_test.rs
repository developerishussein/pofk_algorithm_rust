#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::graph_algorithms::dijkstra::dijkstra;

    #[test]
    fn test_dijkstra_basic() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 1), (3, 4)]);
        graph.insert(2, vec![(3, 2), (4, 5)]);
        graph.insert(3, vec![(4, 1)]);
        graph.insert(4, vec![]);
        let dist = dijkstra(&graph, 1);
        assert_eq!(dist[&4], 4);
        assert_eq!(dist[&3], 3);
    }

    #[test]
    fn test_dijkstra_disconnected() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 1)]);
        graph.insert(2, vec![]);
        graph.insert(3, vec![]);
        let dist = dijkstra(&graph, 1);
        assert_eq!(dist[&2], 1);
        assert!(!dist.contains_key(&3));
    }
}
