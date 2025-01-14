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
        n: usize,
        uv: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut dp = vec![(0, 0); n];

    dfs(0, usize::MAX, 1, &adj, &mut dp);

    for (l, r) in dp {
        println!("{} {}", l, r);
    }
}

fn dfs(v: usize, p: usize, left: usize, adj: &[Vec<usize>], dp: &mut [(usize, usize)]) {
    let mut right = left;

    for &w in &adj[v] {
        if w != p {
            dfs(w, v, right, adj, dp);
            right = dp[w].1 + 1;
        }
    }

    right = (right - 1).max(left);

    dp[v] = (left, right);
}
