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
        n1: usize, n2: usize, m: usize,
        ab: [(usize, usize); m],
    }

    use petgraph::{algo::*, graph::*, prelude::*, visit::*, *};

    let mut graph1 = Graph::new_undirected();
    let mut graph2 = Graph::new_undirected();

    let nodes1 = (0..n1).map(|_| graph1.add_node(())).collect_vec();
    let nodes2 = (0..n2).map(|_| graph2.add_node(())).collect_vec();

    for (a, b) in ab {
        if a <= n1 {
            graph1.add_edge(nodes1[a - 1], nodes1[b - 1], ());
        } else {
            graph2.add_edge(nodes2[a - n1 - 1], nodes2[b - n1 - 1], ());
        }
    }

    let cost1 = dijkstra(&graph1, nodes1[0], None, |_| 1);
    let cost2 = dijkstra(&graph2, nodes2[n2 - 1], None, |_| 1);

    println!(
        "{}",
        cost1.values().max().unwrap() + cost2.values().max().unwrap() + 1
    );
}
