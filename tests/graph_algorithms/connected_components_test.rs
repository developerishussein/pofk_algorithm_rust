#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::graph_algorithms::connected_components::connected_components;

    #[test]
    fn test_connected_components_basic() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![2]);
        graph.insert(2, vec![1]);
        graph.insert(3, vec![4]);
        graph.insert(4, vec![3]);
        let mut comps = connected_components(&graph);
        for comp in &mut comps { comp.sort(); }
        comps.sort();
        assert_eq!(comps, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_connected_components_single() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![]);
        let comps = connected_components(&graph);
        assert_eq!(comps, vec![vec![1]]);
    }
}
