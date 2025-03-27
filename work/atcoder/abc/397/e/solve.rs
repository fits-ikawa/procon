#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
        n: usize, k: usize,
        uv: [(Usize1, Usize1); n*k-1],
    }

    if k == 1 {
        println!("Yes");
        return;
    }

    let mut adj = vec![vec![]; n * k];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    let ans = dfs(0, usize::MAX, &mut vec![0; n * k], k, &adj);

    println!("{}", if ans { "Yes" } else { "No" });
}

fn dfs(v: usize, p: usize, dp: &mut [usize], k: usize, adj: &[Vec<usize>]) -> bool {
    let mut ch = vec![];

    for &w in &adj[v] {
        if w != p {
            if !dfs(w, v, dp, k, adj) {
                return false;
            }
            if dp[w] > 0 {
                ch.push(dp[w]);
            }
        }
    }

    if ch.is_empty() {
        dp[v] = 1;
    } else if ch.len() == 1 {
        dp[v] = (ch[0] + 1) % k;
    } else if ch.len() == 2 {
        if ch[0] + ch[1] + 1 != k {
            return false;
        }
        dp[v] = 0;
    } else {
        return false;
    }

    true
}
