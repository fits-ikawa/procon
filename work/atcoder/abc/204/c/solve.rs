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
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m],
    }

    use petgraph::{algo::*, graph::*, prelude::*, visit::*, *};

    let mut graph = Graph::new();
    let nodes = (0..n).map(|_| graph.add_node(())).collect_vec();

    for (a, b) in ab {
        graph.add_edge(nodes[a], nodes[b], ());
    }

    let mut ans = 0;

    for &start in &nodes {
        let dfs = Dfs::new(&graph, start);

        ans += dfs.iter(&graph).count();
    }

    println!("{}", ans);
}
