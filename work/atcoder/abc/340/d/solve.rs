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
    input! {
        n: usize,
        abx: [(usize, usize, Usize1); n-1],
    }

    use petgraph::{algo::*, graph::*, prelude::*, visit::*, *};

    let mut graph = Graph::new();
    let nodes = (0..n).map(|_| graph.add_node(())).collect_vec();

    for (i, (a, b, x)) in abx.into_iter().enumerate() {
        graph.add_edge(nodes[i], nodes[i + 1], a);
        graph.add_edge(nodes[i], nodes[x], b);
    }

    let cost = dijkstra(&graph, nodes[0], Some(nodes[n - 1]), |e| *e.weight());

    println!("{}", cost[&nodes[n - 1]]);
}
