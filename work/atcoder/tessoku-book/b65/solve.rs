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
        n: usize, t: Usize1,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut ans = vec![0; n];

    dfs(t, &adj, &mut seen, &mut ans);

    println!("{}", ans.iter().join(" "));
}

fn dfs(cur: usize, adj: &[Vec<usize>], seen: &mut [bool], ans: &mut [usize]) {
    seen[cur] = true;
    for &next in &adj[cur] {
        if !seen[next] {
            dfs(next, adj, seen, ans);
            ans[cur] = ans[cur].max(ans[next] + 1);
        }
    }
}
