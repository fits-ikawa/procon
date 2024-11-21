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
    // 優先度付きキューで受け渡しをシミュレートして解く
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut distribution = BinaryHeap::new();
    let mut received = vec![None; n];

    for (i, ti) in t.into_iter().enumerate() {
        distribution.push((Reverse(ti), i));
    }

    while let Some((Reverse(time), i)) = distribution.pop() {
        if received[i].is_none() {
            received[i] = Some(time);
            // 初めて受け取った時のみ次に渡す
            distribution.push((Reverse(time + s[i]), (i + 1) % n));
        }
    }

    println!("{}", received.iter().flatten().join("\n"));
}

#[allow(dead_code)]
fn solve() {
    // グラフ問題とみなしてダイクストラ法で解く
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    use petgraph::{algo::*, graph::*, visit::*, *};

    let mut graph = DiGraph::new();
    let nodes = (0..=n).map(|_| graph.add_node(())).collect_vec();

    for i in 0..n {
        graph.add_edge(nodes[i], nodes[(i + 1) % n], s[i]);
        graph.add_edge(nodes[n], nodes[i], t[i]);
    }

    let costs = dijkstra(&graph, nodes[n], None, |e| *e.weight());

    println!("{}", nodes[..n].iter().map(|node| costs[node]).join("\n"));
}
