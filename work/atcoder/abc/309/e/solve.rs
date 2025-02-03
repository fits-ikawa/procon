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
        n: usize, m: usize,
        p: [Usize1; n-1],
        xy: [(Usize1, isize); m],
    }

    let mut adj = vec![vec![]; n];

    for i in 0..n - 1 {
        adj[p[i]].push(i + 1);
    }

    // dp[i]
    // 人 i の何代先まで保険の補償対象となるか
    // -1 なら人 i は保険に掛けられていない
    let mut dp = vec![-1; n];

    for (x, y) in xy {
        dp[x] = dp[x].max(y);
    }

    dfs(0, &mut dp, &adj);

    println!("{}", dp.iter().filter(|&&x| x >= 0).count());
}

fn dfs(v: usize, dp: &mut [isize], adj: &[Vec<usize>]) {
    for &w in &adj[v] {
        dp[w] = dp[w].max(dp[v] - 1);
        dfs(w, dp, adj);
    }
}
