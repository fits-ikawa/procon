#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use memoise::memoise;
use num::integer::{Integer, Roots};
use petgraph::{algo::*, graph::*, *};
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
    // トポロジカルソート + DP
    input! {
        n: usize, m: usize,
        xy: [(u32, u32); m],
    }

    let mut graph: Graph<(), ()> = DiGraph::new();
    let _nodes = (0..n).map(|_| graph.add_node(())).collect_vec();
    graph.extend_with_edges(xy.into_iter().map(|(x, y)| (x - 1, y - 1)));

    let mut dp = vec![0; n];

    for ni in toposort(&graph, None).unwrap() {
        for nj in graph.neighbors(ni) {
            dp[nj.index()] = dp[nj.index()].max(dp[ni.index()] + 1);
        }
    }

    println!("{}", dp.iter().max().unwrap());
}

#[allow(dead_code)]
fn solve() {
    // メモ化再帰
    input! {
        n: usize, m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];

    for (x, y) in xy {
        adj[x].push(y);
    }

    let mut ans = 0;
    for i in 0..n {
        // rec(i, ...)
        // 頂点 i を始点とした最長パスを求める
        ans = ans.max(rec(i, &adj));
    }

    println!("{}", ans);
}

#[allow(dead_code)]
#[memoise(v < 100000)]
fn rec(v: usize, adj: &[Vec<usize>]) -> usize {
    let mut ret = 0;
    for &to in &adj[v] {
        ret = ret.max(rec(to, adj) + 1);
    }
    return ret;
}
