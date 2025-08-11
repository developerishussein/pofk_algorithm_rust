#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::graph_algorithms::bellman_ford::bellman_ford;

    #[test]
    fn test_bellman_ford_basic() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 1), (3, 4)]);
        graph.insert(2, vec![(3, -2), (4, 5)]);
        graph.insert(3, vec![(4, 1)]);
        graph.insert(4, vec![]);
        let dist = bellman_ford(&graph, 1).unwrap();
        assert_eq!(dist[&4], 0);
    }

    #[test]
    fn test_bellman_ford_negative_cycle() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(2, 1)]);
        graph.insert(2, vec![(1, -2)]);
        assert!(bellman_ford(&graph, 1).is_none());
    }
}
