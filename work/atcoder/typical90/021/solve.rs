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
        ab: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];
    let mut adj_rev = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj_rev[b].push(a);
    }

    let mut seen = vec![false; n];
    let mut order = vec![];

    for v in 0..n {
        if !seen[v] {
            dfs(v, &mut seen, &mut order, &adj);
        }
    }

    let mut seen = vec![false; n];
    let mut ans = 0;

    for &v in order.iter().rev() {
        let mut group = vec![];
        dfs(v, &mut seen, &mut group, &adj_rev);
        ans += group.len() * (group.len() - 1) / 2;
    }

    println!("{}", ans);
}

fn dfs(cur: usize, seen: &mut Vec<bool>, ans: &mut Vec<usize>, adj: &[Vec<usize>]) {
    seen[cur] = true;

    for &next in &adj[cur] {
        if !seen[next] {
            dfs(next, seen, ans, adj);
        }
    }

    ans.push(cur);
}
