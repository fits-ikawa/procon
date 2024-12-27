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

    let mut seen = vec![None; n];
    let mut ans = n * (n - 1) / 2 - m;

    for i in 0..n {
        if seen[i].is_none() {
            let mut cnt = [0, 0];
            if dfs(i, false, &mut cnt, &mut seen, &adj) {
                ans -=
                    cnt[0] * cnt[0].saturating_sub(1) / 2 + cnt[1] * cnt[1].saturating_sub(1) / 2;
            } else {
                // 連結成分のどれか一つでも二部グラフでなければ、
                // どの辺を繋げても二部グラフにならない
                println!("0");
                return;
            }
        }
    }

    println!("{}", ans);
}

fn dfs(
    v: usize,
    color: bool,
    cnt: &mut [usize; 2],
    seen: &mut [Option<bool>],
    adj: &[Vec<usize>],
) -> bool {
    seen[v] = Some(color);
    cnt[if color { 1 } else { 0 }] += 1;

    let mut ret = true;

    for &w in &adj[v] {
        if let Some(wcolor) = seen[w] {
            if color == wcolor {
                ret = false;
            }
        } else {
            ret &= dfs(w, !color, cnt, seen, adj);
        }
    }

    ret
}
