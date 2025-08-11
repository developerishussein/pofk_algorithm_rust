#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::graph_algorithms::topological_sort::topological_sort;

    #[test]
    fn test_topological_sort_basic() {
        let mut graph = HashMap::new();
        graph.insert(5, vec![2, 0]);
        graph.insert(4, vec![0, 1]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![1]);
        graph.insert(0, vec![]);
        graph.insert(1, vec![]);
        let order = topological_sort(&graph).unwrap();
        assert_eq!(order.len(), 6);
        // Check order validity
        let pos: HashMap<_, _> = order.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        for (u, neighbors) in &graph {
            for v in neighbors {
                assert!(pos[u] < pos[v]);
            }
        }
    }

    #[test]
    fn test_topological_sort_cycle() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![1]);
        assert!(topological_sort(&graph).is_none());
    }
}
