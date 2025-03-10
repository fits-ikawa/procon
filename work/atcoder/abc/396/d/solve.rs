#![allow(clippy::comparison_chain)]
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
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut adj = vec![vec![]; n];

    for (u, v, w) in uvw {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut ans = usize::MAX;

    dfs(0, 0, &mut hashset! {0}, &mut ans, n, &adj);

    println!("{}", ans);
}

fn dfs(
    cur: usize,
    xor: usize,
    seen: &mut HashSet<usize>,
    ans: &mut usize,
    n: usize,
    adj: &[Vec<(usize, usize)>],
) {
    if cur == n - 1 {
        *ans = (*ans).min(xor);
        return;
    }

    seen.insert(cur);

    for &(v, w) in &adj[cur] {
        if !seen.contains(&v) {
            dfs(v, xor ^ w, seen, ans, n, adj);
        }
    }

    seen.remove(&cur);
}
