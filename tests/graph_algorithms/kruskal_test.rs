// Tests for Kruskal's Minimum Spanning Tree (MST) algorithm
use super::super::graph_algorithms::kruskal::kruskal;

#[test]
fn test_kruskal_basic() {
    let nodes = [1, 2, 3, 4];
    let edges = [
        (1, 2, 1),
        (2, 3, 2),
        (3, 4, 1),
        (4, 1, 2),
        (1, 3, 2),
    ];
    let mut mst = kruskal(&nodes, &edges);
    mst.sort();
    let mut expected = vec![(1, 2, 1), (3, 4, 1), (2, 3, 2)];
    expected.sort();
    assert_eq!(mst, expected);
}

#[test]
fn test_kruskal_disconnected() {
    let nodes = [1, 2, 3, 4, 5];
    let edges = [
        (1, 2, 1),
        (2, 3, 2),
        (4, 5, 1),
    ];
    let mut mst = kruskal(&nodes, &edges);
    mst.sort();
    let mut expected = vec![(1, 2, 1), (2, 3, 2), (4, 5, 1)];
    expected.sort();
    assert_eq!(mst, expected);
}

#[test]
fn test_kruskal_string_nodes() {
    let nodes = ["A", "B", "C"];
    let edges = [
        ("A", "B", 2),
        ("B", "C", 1),
        ("A", "C", 3),
    ];
    let mut mst = kruskal(&nodes, &edges);
    mst.sort();
    let mut expected = vec![("A", "B", 2), ("B", "C", 1)];
    expected.sort();
    assert_eq!(mst, expected);
}
