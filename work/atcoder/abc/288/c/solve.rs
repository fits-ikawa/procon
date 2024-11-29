#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use proconio::{marker::*, *};
use std::cmp::{Ordering::*, Reverse};
use std::collections::*;
use superslice::*;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

#[allow(clippy::needless_range_loop)]
#[fastout]
fn main() {
    // UnionFind
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut min_edges = 0;

    for &(a, b) in &ab {
        if uf.leader(a) != uf.leader(b) {
            uf.merge(a, b);
            min_edges += 1;
        }
    }

    println!("{}", ab.len() - min_edges);
}

#[allow(dead_code)]
fn solve() {
    // 最小全域木（オーバーキル？）
    input! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }

    use petgraph::{algo::*, graph::*, prelude::*, visit::*, *};

    let mut graph = Graph::new_undirected();
    let nodes = (0..n).map(|_| graph.add_node(())).collect_vec();

    for &(a, b) in &ab {
        graph.add_edge(nodes[a], nodes[b], ());
    }

    let min_edges = min_spanning_tree(&graph)
        .filter(|x| {
            matches!(
                x,
                petgraph::data::Element::Edge {
                    source: _,
                    target: _,
                    weight: _
                }
            )
        })
        .collect_vec();

    println!("{}", ab.len() - min_edges.len());
}
