#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use petgraph::{algo::*, graph::*, prelude::*, visit::*, *};
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
    input! {
        n: usize, m: usize,
        a: [usize; n],
        uvb: [(Usize1, Usize1, usize); m],
    }

    let mut graph = Graph::new_undirected();
    let nodes = a.iter().map(|&ai| graph.add_node(ai)).collect_vec();
    for &(u, v, b) in &uvb {
        graph.add_edge(nodes[u], nodes[v], b);
    }

    let cost = dijkstra(&graph, nodes[0], None, |e| *e.weight() + graph[e.target()]);

    println!(
        "{}",
        nodes[1..]
            .iter()
            .map(|node| cost[node] + graph[nodes[0]])
            .join(" ")
    );
}
