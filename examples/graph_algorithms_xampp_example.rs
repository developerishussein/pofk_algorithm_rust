// Example usage of new graph algorithms for XAMPP-style integration
// This file demonstrates all new algorithms in a single Rust main function.
// You can run this as a Rust binary, or adapt the code for FFI/WebAssembly integration.

use pofk_algorithm::graph_algorithms::kruskal::kruskal;
use pofk_algorithm::graph_algorithms::prim::prim;
use pofk_algorithm::graph_algorithms::kosaraju_scc::kosaraju_scc;
use pofk_algorithm::graph_algorithms::articulation_points::articulation_points;

fn main() {
    use std::collections::HashMap;
    // Kruskal's MST
    let nodes = [1, 2, 3, 4];
    let edges = [
        (1, 2, 1),
        (2, 3, 2),
        (3, 4, 1),
        (4, 1, 2),
        (1, 3, 2),
    ];
    let mst = kruskal(&nodes, &edges);
    println!("Kruskal MST: {:?}", mst);

    // Prim's MST
    let mst_prim = prim(&nodes, &edges, 1);
    println!("Prim MST: {:?}", mst_prim);

    // Kosaraju's SCC
    let nodes_scc = [1, 2, 3, 4, 5];
    let edges_scc = [
        (1, 2), (2, 3), (3, 1),
        (3, 4), (4, 5), (5, 4)
    ];
    let mut sccs = kosaraju_scc(&nodes_scc, &edges_scc);
    for scc in &mut sccs { scc.sort(); }
    sccs.sort();
    println!("Kosaraju SCCs: {:?}", sccs);

    // Articulation Points
    let nodes_ap = [1, 2, 3, 4, 5];
    let edges_ap = [
        (1, 2), (2, 3), (3, 4), (4, 5), (3, 5)
    ];
    let mut points = articulation_points(&nodes_ap, &edges_ap);
    points.sort();
    println!("Articulation Points: {:?}", points);

    // DFS
    let mut graph = HashMap::new();
    graph.insert(1, vec![2, 3]);
    graph.insert(2, vec![4]);
    graph.insert(3, vec![]);
    graph.insert(4, vec![]);
    let dfs_order = pofk_algorithm::graph_algorithms::dfs::dfs(&graph, 1);
    println!("DFS order: {:?}", dfs_order);

    // BFS
    let bfs_order = pofk_algorithm::graph_algorithms::bfs::bfs(&graph, 1);
    println!("BFS order: {:?}", bfs_order);

    // Dijkstra
    let mut wgraph = HashMap::new();
    wgraph.insert(1, vec![(2, 1), (3, 4)]);
    wgraph.insert(2, vec![(3, 2), (4, 5)]);
    wgraph.insert(3, vec![(4, 1)]);
    wgraph.insert(4, vec![]);
    let dist = pofk_algorithm::graph_algorithms::dijkstra::dijkstra(&wgraph, 1);
    println!("Dijkstra distances: {:?}", dist);

    // Bellman-Ford
    let mut bf_graph = HashMap::new();
    bf_graph.insert(1, vec![(2, 1), (3, 4)]);
    bf_graph.insert(2, vec![(3, -2), (4, 5)]);
    bf_graph.insert(3, vec![(4, 1)]);
    bf_graph.insert(4, vec![]);
    let bf_dist = pofk_algorithm::graph_algorithms::bellman_ford::bellman_ford(&bf_graph, 1);
    println!("Bellman-Ford distances: {:?}", bf_dist);

    // Floyd-Warshall
    let nodes_fw = [1, 2, 3, 4];
    let edges_fw = [
        (1, 2, 1),
        (2, 3, 2),
        (1, 3, 4),
        (3, 4, 1),
    ];
    let fw_dist = pofk_algorithm::graph_algorithms::floyd_warshall::floyd_warshall(&nodes_fw, &edges_fw);
    println!("Floyd-Warshall distances: {:?}", fw_dist);

    // Topological Sort
    let mut dag = HashMap::new();
    dag.insert(5, vec![2, 0]);
    dag.insert(4, vec![0, 1]);
    dag.insert(2, vec![3]);
    dag.insert(3, vec![1]);
    dag.insert(0, vec![]);
    dag.insert(1, vec![]);
    let topo = pofk_algorithm::graph_algorithms::topological_sort::topological_sort(&dag);
    println!("Topological sort: {:?}", topo);

    // Union-Find
    let mut uf = pofk_algorithm::graph_algorithms::union_find::UnionFind::new();
    uf.add(1); uf.add(2); uf.add(3);
    uf.union(1, 2);
    println!("Union-Find connected(1,2): {}", uf.connected(1, 2));
    println!("Union-Find connected(1,3): {}", uf.connected(1, 3));

    // Connected Components
    let mut cc_graph = HashMap::new();
    cc_graph.insert(1, vec![2]);
    cc_graph.insert(2, vec![1]);
    cc_graph.insert(3, vec![4]);
    cc_graph.insert(4, vec![3]);
    let mut comps = pofk_algorithm::graph_algorithms::connected_components::connected_components(&cc_graph);
    for comp in &mut comps { comp.sort(); }
    comps.sort();
    println!("Connected components: {:?}", comps);

    // Cycle Detection
    let mut cycle_graph = HashMap::new();
    cycle_graph.insert(1, vec![2]);
    cycle_graph.insert(2, vec![3]);
    cycle_graph.insert(3, vec![1]);
    println!("Has cycle: {}", pofk_algorithm::graph_algorithms::cycle_detection::has_cycle(&cycle_graph));
    cycle_graph.insert(3, vec![]);
    println!("Has cycle (after removal): {}", pofk_algorithm::graph_algorithms::cycle_detection::has_cycle(&cycle_graph));

    // Bipartite Graph
    let mut bipartite_graph = HashMap::new();
    bipartite_graph.insert(1, vec![2, 3]);
    bipartite_graph.insert(2, vec![1, 4]);
    bipartite_graph.insert(3, vec![1, 4]);
    bipartite_graph.insert(4, vec![2, 3]);
    println!("Is bipartite: {}", pofk_algorithm::graph_algorithms::bipartite_graph::is_bipartite(&bipartite_graph));
    bipartite_graph.insert(4, vec![2, 3, 1]);
    println!("Is bipartite (after making non-bipartite): {}", pofk_algorithm::graph_algorithms::bipartite_graph::is_bipartite(&bipartite_graph));
}
