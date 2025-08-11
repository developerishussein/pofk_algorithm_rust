#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::graph_algorithms::cycle_detection::has_cycle;

    #[test]
    fn test_cycle_detection_true() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![1]);
        assert!(has_cycle(&graph));
    }

    #[test]
    fn test_cycle_detection_false() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![3]);
        graph.insert(3, vec![]);
        assert!(!has_cycle(&graph));
    }
}
