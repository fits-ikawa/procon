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
        n: usize, x: Usize1, y: Usize1,
        uv: [(Usize1, Usize1); n - 1],
    }

    let mut adj = vec![vec![]; n];
    for &(u, v) in &uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    // BFS
    let mut todo = VecDeque::new();
    let mut seen = vec![false; n];
    let mut prev = vec![None; n];
    todo.push_back(x);
    seen[x] = true;

    while let Some(from) = todo.pop_front() {
        for &to in &adj[from] {
            if !seen[to] {
                seen[to] = true;
                prev[to] = Some(from);
                todo.push_back(to);
            }
        }
    }

    let mut ans = vec![y + 1];
    let mut cur = y;
    while let Some(p) = prev[cur] {
        ans.push(p + 1);
        cur = p;
    }

    println!("{}", ans.iter().rev().join(" "));
}
