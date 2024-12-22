#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    use petgraph::{algo::*, graph::*, prelude::*, visit::*, *};

    let mut graph = Graph::new_undirected();
    let nodes = (0..n).map(|_| graph.add_node(())).collect_vec();

    for (a, b, c) in abc {
        graph.add_edge(nodes[a], nodes[b], c);
    }

    let cost_1 = dijkstra(&graph, nodes[0], None, |e| *e.weight());
    let cost_n = dijkstra(&graph, nodes[n - 1], None, |e| *e.weight());

    for i in 0..n {
        let ans = cost_1[&(nodes[i])] + cost_n[&(nodes[i])];
        println!("{}", ans);
    }
}
