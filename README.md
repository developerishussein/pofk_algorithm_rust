# 🧠 POFK Algorithm (Rust Edition)

![Temporary Logo](https://github.com/POFKLabs/pofk_algorithm/blob/main/logo/logo.jpg)

> A comprehensive, fast, and extensible algorithms library for Rust. Includes classic and modern techniques for lists, sets, maps, strings, graphs, and more — built with idiomatic Rust, strong typing, and clean APIs.

---

- Actively maintained with ambitious roadmap (1,000+ algorithms planned)  
- Type-safe generics across the library  
- Readable, idiomatic Rust code with clear documentation and tests  

Environment: Rust 1.70+ (see `Cargo.toml`).

---

## 📦 Install

Add to your `Cargo.toml`:

```toml
[dependencies]
pofk_algorithm = "0.0.1"
```
## 🚀 Quick start
```rust

use pofk_algorithm::*;

use pofk_algorithm::{
    list_algorithms::{binary_search, merge_sort},
    string_algorithms::{is_palindrome, longest_common_prefix},
    graph_algorithms::{dijkstra, WeightedEdge},
};

fn main() {
    // List algorithms
    let arr = vec![1, 3, 5, 7, 9];
    let idx = binary_search(&arr, 7);
    let sorted = merge_sort(vec![5, 2, 4, 6, 1, 3]);

    // String algorithms
    let is_pal = is_palindrome("A man a plan a canal Panama");
    let lcp = longest_common_prefix(&["flower", "flow", "flight"]);

    // Graph algorithms
    let graph = vec![
        ("A", vec![WeightedEdge::new("A", "B", 1), WeightedEdge::new("A", "C", 4)]),
        ("B", vec![WeightedEdge::new("B", "C", 2)]),
        ("C", vec![]),
    ].into_iter().collect();

    let dist = dijkstra(&graph, "A");

    println!("{:?}", (idx, sorted, is_pal, lcp, dist));
}
```


---

## 🧩 Algorithms included

### List algorithms
- binary_search, linear_search
- merge_sort, quick_sort, bubble_sort, insertion_sort, selection_sort
- counting_sort (non-negative ints)
- reverse_list, find_max_min, find_duplicates, remove_duplicates
- kadanes_algorithm
- max_sum_subarray_of_size_k, min_sum, average_subarray, prefix_sum, two_sum_sorted
- rotate_array_right

### Set algorithms
- has_duplicates, has_two_sum, has_unique_window
- disjoint_set (Union-Find), find_intersection, set_difference, is_frequency_unique

### Map algorithms
- frequency_count, most_frequent_element, top_k_frequent
- group_by_key, first_non_repeated_element
- anagram_checker (generic list-based)
- two_sum (indices), lru_cache, length_of_longest_substring

### String algorithms
- reverse_string, palindrome_checker, anagram_checker
- brute_force_search, kmp_search, rabin_karp_search
- longest_common_prefix, longest_palindromic_substring
- edit_distance, string_compression, count_vowels_consonants

### Graph algorithms (new)
- bfs, dfs, topological_sort
- connected_components, cycle_detection (directed/undirected), bipartite_graph
- shortest_path (unweighted BFS), weighted_edge (utility)
- dijkstra, bellman_ford, floyd_warshall
- mst_kruskal, mst_prim
- kosaraju_scc, articulation_points, union_find (typedef)

Each function includes Dartdoc with usage and time/space complexity.

---

## 📚 Usage notes

- Import everything via use `pofk_algorithm::*;`
- Sorting/searching functions use `T extends Comparable` where appropriate.
- Weighted graph utilities use `WeightedEdge<T>`.
- Algorithms are pure and side-effect free unless documented otherwise.

---

## 🧪 Running tests

```bash
cargo test
```

All tests pass in the repository (see `test/`).

---

## 🤝 Contributing

Contributions are welcome!
- Add new algorithms or optimize existing ones
- Improve docs and examples
- Increase test coverage

Open a PR with a brief description and test cases.

---

## 🗺️ Roadmap (short-term)
- Expand graph algorithms (SPFA, Johnson, Edmonds-Karp, Dinic)
- Add tree/heap/DP/geometry modules
- Benchmarks and performance docs

---

## 📄 License

MIT. See `LICENSE`.