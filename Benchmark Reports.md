# Benchmark Reports

This document contains benchmark results for the entire `pofk_algorithms` Rust library, including execution time and memory usage for representative algorithms in each module.

## How to Run Benchmarks

1. Ensure you are in the project root.
2. Run all benchmarks:
   ```sh
   cargo bench
   ```
3. For memory profiling (Linux/macOS):
   - Install `valgrind` or `massif`.
   - Example: `cargo install cargo-valgrind`
   - Run: `cargo valgrind --bench benchmark`
   
   For Windows, use [Windows Performance Toolkit](https://docs.microsoft.com/en-us/windows-hardware/test/wpt/) or [Dr. Memory](http://www.drmemory.org/).

## Benchmark Results

*This section will be updated with actual results after running the benchmarks.*

| Algorithm                | Time (ns/iter) | Memory (KB) |
|--------------------------|----------------|-------------|
| List: Merge Sort         |                |             |
| Set: Union               |                |             |
| String: Rabin-Karp       |                |             |
| Graph: Dijkstra          |                |             |
| Tree: LCA                |                |             |
| Linked List: Reverse     |                |             |
| DP: Knapsack             |                |             |
| Backtracking: N-Queens   |                |             |
| Matrix: Flood Fill       |                |             |

---

## Notes
- Benchmarks use `criterion` for high-precision timing.
- Memory usage is measured using `heaptrack`, `valgrind`, or platform-specific tools.
- See `benches/benchmark.rs` for implementation details.
