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
        n: usize, q: usize,
        x: [usize; n],
        ab: [(Usize1, Usize1); n-1],
        vk: [(Usize1, Usize1); q],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut dp = vec![vec![]; n];

    dfs(0, usize::MAX, &adj, &mut dp, &x);

    for (v, k) in vk {
        println!("{}", dp[v][k]);
    }
}

fn dfs(v: usize, p: usize, adj: &[Vec<usize>], dp: &mut [Vec<usize>], x: &[usize]) {
    let mut xs = vec![x[v]];

    for &w in &adj[v] {
        if w != p {
            dfs(w, v, adj, dp, x);
            xs.extend_from_slice(&dp[w]);
        }
    }

    xs.sort();
    xs.reverse();

    dp[v] = xs.into_iter().take(20).collect();
}
