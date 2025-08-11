// Tests for Prim's Minimum Spanning Tree (MST) algorithm
use super::super::graph_algorithms::prim::prim;

#[test]
fn test_prim_basic() {
    let nodes = [1, 2, 3, 4];
    let edges = [
        (1, 2, 1),
        (2, 3, 2),
        (3, 4, 1),
        (4, 1, 2),
        (1, 3, 2),
    ];
    let mut mst = prim(&nodes, &edges, 1);
    mst.sort();
    let mut expected = vec![(1, 2, 1), (3, 4, 1), (1, 3, 2)];
    expected.sort();
    assert_eq!(mst, expected);
}

#[test]
fn test_prim_disconnected() {
    let nodes = [1, 2, 3, 4, 5];
    let edges = [
        (1, 2, 1),
        (2, 3, 2),
        (4, 5, 1),
    ];
    let mut mst = prim(&nodes, &edges, 1);
    mst.sort();
    let mut expected = vec![(1, 2, 1), (2, 3, 2)];
    expected.sort();
    assert_eq!(mst, expected);
    // Test the other component
    let mut mst2 = prim(&nodes, &edges, 4);
    mst2.sort();
    let mut expected2 = vec![(4, 5, 1)];
    expected2.sort();
    assert_eq!(mst2, expected2);
}

#[test]
fn test_prim_string_nodes() {
    let nodes = ["A", "B", "C"];
    let edges = [
        ("A", "B", 2),
        ("B", "C", 1),
        ("A", "C", 3),
    ];
    let mut mst = prim(&nodes, &edges, "A");
    mst.sort();
    let mut expected = vec![("A", "B", 2), ("B", "C", 1)];
    expected.sort();
    assert_eq!(mst, expected);
}
