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
        ab: [(Usize1, Usize1); n-1],
        v: [Usize1; k],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    let vs = v.iter().copied().collect::<HashSet<_>>();

    println!("{}", dfs(v[0], usize::MAX, &adj, &vs));
}

fn dfs(v: usize, p: usize, adj: &[Vec<usize>], vs: &HashSet<usize>) -> usize {
    let mut ret = 0;

    for &w in &adj[v] {
        if w != p {
            ret += dfs(w, v, adj, vs);
        }
    }

    if ret > 0 || vs.contains(&v) {
        ret += 1;
    }

    ret
}
