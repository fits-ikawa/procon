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
        uv: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut seen = hashset! {};
    let mut ans = 0;

    dfs(0, &mut seen, &mut ans, &adj);

    println!("{}", ans);
}

fn dfs(v: usize, seen: &mut HashSet<usize>, ans: &mut usize, adj: &[Vec<usize>]) {
    if *ans >= 1000000 {
        return;
    }

    *ans += 1;
    seen.insert(v);

    for &w in &adj[v] {
        if !seen.contains(&w) {
            dfs(w, seen, ans, adj);
        }
    }

    seen.remove(&v);
}
