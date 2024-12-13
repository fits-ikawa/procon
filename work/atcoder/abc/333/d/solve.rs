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
    // UnionFind で部分木の頂点数をカウント
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n-1],
    }

    let mut uf = ac_library::Dsu::new(n);
    let mut cnt = 0;

    for (u, v) in uv {
        if u == 0 || v == 0 {
            cnt += 1;
        } else {
            uf.merge(u, v);
        }
    }

    let ans = uf
        .groups()
        .iter()
        .map(|g| g.len())
        .sorted()
        .take(cnt)
        .sum::<usize>();

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // BFS で部分木の頂点数をカウント
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    if adj[0].len() == 1 {
        println!("1");
        return;
    }

    let ans = adj[0]
        .iter()
        .map(|&i| bfs(i, &adj, n))
        .sorted()
        .take(adj[0].len() - 1)
        .sum::<usize>()
        + 1;

    println!("{}", ans);
}

#[allow(dead_code)]
fn bfs(start: usize, adj: &[Vec<usize>], n: usize) -> usize {
    let mut todo = VecDeque::new();
    let mut seen = vec![false; n];
    todo.push_back(start);
    seen[0] = true;
    seen[start] = true;

    let mut count = 0;

    while let Some(from) = todo.pop_front() {
        count += 1;
        for &to in &adj[from] {
            if !seen[to] {
                seen[to] = true;
                todo.push_back(to);
            }
        }
    }

    count
}
