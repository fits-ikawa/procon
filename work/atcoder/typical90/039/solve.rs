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
        ab: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    // dp[i]
    // 頂点 0 を根とした木において、頂点 i を根とする部分木のサイズ
    let mut dp = vec![0; n];

    dfs(0, usize::MAX, &mut dp, &adj);

    let mut ans = 0;

    for i in 0..n {
        ans += dp[i] * (n - dp[i]);
    }

    println!("{}", ans);
}

fn dfs(v: usize, p: usize, dp: &mut [usize], adj: &[Vec<usize>]) {
    dp[v] = 1;

    for &w in &adj[v] {
        if w != p {
            dfs(w, v, dp, adj);
            dp[v] += dp[w];
        }
    }
}
