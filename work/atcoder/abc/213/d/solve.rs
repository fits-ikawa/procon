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
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![BTreeSet::new(); n];

    for (a, b) in ab {
        adj[a].insert(b);
        adj[b].insert(a);
    }

    let mut seen = vec![false; n];
    let mut ans = vec![];

    dfs(0, &adj, &mut seen, &mut ans);

    println!("{}", ans.iter().join(" "));
}

fn dfs(node: usize, adj: &[BTreeSet<usize>], seen: &mut [bool], ans: &mut Vec<usize>) {
    ans.push(node + 1);
    seen[node] = true;
    for &to in &adj[node] {
        if !seen[to] {
            dfs(to, adj, seen, ans);
            ans.push(node + 1);
        }
    }
}
