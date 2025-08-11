#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::graph_algorithms::floyd_warshall::floyd_warshall;

    #[test]
    fn test_floyd_warshall_basic() {
        let nodes = [1, 2, 3, 4];
        let edges = [
            (1, 2, 1),
            (2, 3, 2),
            (1, 3, 4),
            (3, 4, 1),
        ];
        let dist = floyd_warshall(&nodes, &edges);
        assert_eq!(dist[&(1, 4)], 4);
        assert_eq!(dist[&(1, 3)], 3);
    }
}
