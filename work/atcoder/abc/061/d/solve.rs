#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use num::integer::{Integer, Roots};
use petgraph::{algo::*, graph::*, visit::*, *};
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
        abc: [(u32, u32, f64); m],
    }

    let abc = abc
        .into_iter()
        .map(|(a, b, c)| (a - 1, b - 1, c))
        .collect_vec();

    // 終点へ到達可能な頂点を抽出する
    let mut graph = DiGraph::new();
    let node = (0..n).map(|_| graph.add_node(())).collect_vec();
    graph.extend_with_edges(abc.iter().map(|&(a, b, _)| (b, a, ())));

    let mut reachables = hashset![];
    let mut dfs = Dfs::new(&graph, node[n - 1]);

    while let Some(node) = dfs.next(&graph) {
        reachables.insert(node.index() as u32);
    }

    // 終点へ到達不可能な頂点を含む辺を除外して問題のグラフを構築する。
    // 最長路の長さを求めるので重みの符号は反転
    let mut graph = DiGraph::new();
    let node = (0..n).map(|_| graph.add_node(())).collect_vec();

    graph.extend_with_edges(abc.iter().filter_map(|&(a, b, c)| {
        if reachables.contains(&a) && reachables.contains(&b) {
            Some((a, b, -c))
        } else {
            None
        }
    }));

    // 始点からベルマン・フォード法を適用し、終点への距離を求める
    match bellman_ford(&graph, node[0]) {
        Ok(path) => println!("{}", -path.distances[n - 1] as isize),
        Err(_) => println!("inf"), // 始点 -> 負閉路 -> 終点 の経路が存在した
    }
}
