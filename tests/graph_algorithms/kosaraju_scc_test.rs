// Tests for Kosaraju's Strongly Connected Components (SCC) algorithm
use super::super::graph_algorithms::kosaraju_scc::kosaraju_scc;

#[test]
fn test_kosaraju_scc_basic() {
    let nodes = [1, 2, 3, 4, 5];
    let edges = [
        (1, 2), (2, 3), (3, 1),
        (3, 4), (4, 5), (5, 4)
    ];
    let mut sccs = kosaraju_scc(&nodes, &edges);
    for scc in &mut sccs { scc.sort(); }
    sccs.sort();
    assert_eq!(sccs, vec![vec![1,2,3], vec![4,5]]);
}

#[test]
fn test_kosaraju_scc_singletons() {
    let nodes = [1, 2, 3];
    let edges = [];
    let mut sccs = kosaraju_scc(&nodes, &edges);
    for scc in &mut sccs { scc.sort(); }
    sccs.sort();
    assert_eq!(sccs, vec![vec![1], vec![2], vec![3]]);
}

#[test]
fn test_kosaraju_scc_strings() {
    let nodes = ["A", "B", "C", "D"];
    let edges = [
        ("A", "B"), ("B", "A"), ("C", "D"), ("D", "C")
    ];
    let mut sccs = kosaraju_scc(&nodes, &edges);
    for scc in &mut sccs { scc.sort(); }
    sccs.sort();
    assert_eq!(sccs, vec![vec!["A", "B"], vec!["C", "D"]]);
}
