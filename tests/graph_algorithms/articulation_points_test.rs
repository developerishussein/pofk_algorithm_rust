// Tests for Articulation Points (Cut Vertices) algorithm
use super::super::graph_algorithms::articulation_points::articulation_points;

#[test]
fn test_articulation_points_basic() {
    let nodes = [1, 2, 3, 4, 5];
    let edges = [
        (1, 2), (2, 3), (3, 4), (4, 5), (3, 5)
    ];
    let mut points = articulation_points(&nodes, &edges);
    points.sort();
    assert_eq!(points, vec![3, 4]);
}

#[test]
fn test_articulation_points_none() {
    let nodes = [1, 2, 3];
    let edges = [
        (1, 2), (2, 3), (3, 1)
    ];
    let points = articulation_points(&nodes, &edges);
    assert!(points.is_empty());
}

#[test]
fn test_articulation_points_strings() {
    let nodes = ["A", "B", "C", "D"];
    let edges = [
        ("A", "B"), ("B", "C"), ("C", "D")
    ];
    let mut points = articulation_points(&nodes, &edges);
    points.sort();
    assert_eq!(points, vec!["B", "C"]);
}
