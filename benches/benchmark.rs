//! Library-wide benchmarks for pofk_algorithms
//! Run with: `cargo bench`

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use pofk_algorithms::*;

fn bench_merge_sort(c: &mut Criterion) {
    let data = (0..10_000).rev().collect::<Vec<_>>();
    c.bench_function("list::merge_sort", |b| {
        b.iter(|| {
            let mut arr = data.clone();
            list_algorithms::merge_sort::merge_sort(&mut arr);
            black_box(&arr);
        })
    });
}

fn bench_set_union(c: &mut Criterion) {
    let a = vec![1,2,3,4,5];
    let b = vec![4,5,6,7,8];
    c.bench_function("set::find_union", move |bencher| {
        let a = a.clone();
        let b = b.clone();
        bencher.iter(|| {
            let a = a.clone();
            let b = b.clone();
            let _ = set_algorithms::find_union::find_union(&a, &b);
        })
    });
}

fn bench_rabin_karp(c: &mut Criterion) {
    let text = "a".repeat(100_000) + "needle";
    let pattern = "needle";
    c.bench_function("string::rabin_karp", |b| {
        b.iter(|| {
            let _ = string_algorithms::rabin_karp::rabin_karp(&text, pattern);
        })
    });
}

fn bench_dijkstra(c: &mut Criterion) {
    use std::collections::HashMap;
    let mut graph = HashMap::new();
    graph.insert(1, vec![(2, 1), (3, 4)]);
    graph.insert(2, vec![(3, 2), (4, 5)]);
    graph.insert(3, vec![(4, 1)]);
    graph.insert(4, vec![]);
    c.bench_function("graph::dijkstra", |b| {
        b.iter(|| {
            let _ = graph_algorithms::dijkstra::dijkstra(&graph, 1);
        })
    });
}

fn bench_lca(c: &mut Criterion) {
    use pofk_algorithms::tree_algorithms::binary_tree_traversal::TreeNode;
    let root = Some(Box::new(TreeNode::new(1)));
    c.bench_function("tree::lowest_common_ancestor", |b| {
        b.iter(|| {
            let _ = tree_algorithms::lowest_common_ancestor::lowest_common_ancestor(&root, &1, &1);
        })
    });
}

fn bench_reverse_list(c: &mut Criterion) {
    use pofk_algorithms::linked_list_algorithms::singly_linked_list::ListNode;
    let mut head = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    c.bench_function("linked_list::reverse_list", |b| {
        b.iter(|| {
            let _ = linked_list_algorithms::reverse_list::reverse_list(head.clone());
        })
    });
}

fn bench_knapsack(c: &mut Criterion) {
    let weights = vec![2usize, 3, 4, 5];
    let values = vec![3usize, 4, 5, 6];
    let capacity = 5usize;
    c.bench_function("dp::knapsack_01", |b| {
        b.iter(|| {
            let _ = dp_algorithms::knapsack_01::knapsack_01(&weights, &values, capacity);
        })
    });
}

fn bench_n_queens(c: &mut Criterion) {
    c.bench_function("backtracking::n_queens", |b| {
        b.iter(|| {
            let _ = backtracking_algorithms::n_queens::n_queens(8usize);
        })
    });
}

fn bench_flood_fill(c: &mut Criterion) {
    let grid = vec![vec![1u8; 20]; 20];
    c.bench_function("matrix::flood_fill", |b| {
        b.iter(|| {
            let mut g = grid.clone();
            matrix_algorithms::flood_fill::flood_fill(&mut g, 0, 0, 2u8);
        })
    });
}

criterion_group!(benches,
    bench_merge_sort,
    bench_set_union,
    bench_rabin_karp,
    bench_dijkstra,
    bench_lca,
    bench_reverse_list,
    bench_knapsack,
    bench_n_queens,
    bench_flood_fill,
);
criterion_main!(benches);
