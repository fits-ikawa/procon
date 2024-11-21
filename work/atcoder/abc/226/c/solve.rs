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
    }

    let mut t = vec![];
    let mut k = vec![];
    let mut a = vec![];

    for _ in 0..n {
        input! {
            t_in: usize, k_in: usize, a_in: [Usize1; k_in],
        }
        t.push(t_in);
        k.push(k_in);
        a.push(a_in);
    }

    use petgraph::{algo::*, graph::*, prelude::*, visit::*, *};

    let mut graph = Graph::new();
    let nodes = (0..n).map(|_| graph.add_node(())).collect_vec();
    for i in 0..n {
        for &ai in &a[i] {
            graph.add_edge(nodes[i], nodes[ai], ());
        }
    }

    // 技 N から到達可能な技が覚える必要のある技
    let mut dfs = Dfs::new(&graph, nodes[n - 1]);
    let mut need = hashset! {n - 1};

    while let Some(node) = dfs.next(&graph) {
        need.insert(node.index());
    }

    println!("{}", need.iter().map(|&i| t[i]).sum::<usize>());
}
