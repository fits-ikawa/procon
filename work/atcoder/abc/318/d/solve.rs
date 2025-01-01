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
    // bitDP （解説 AC）
    input! {
        n: usize,
    }

    let mut d = vec![];

    for i in 0..n - 1 {
        input! {
            di: [usize; n - i - 1],
        }

        d.push(di);
    }

    let mut adj = vec![vec![0; n]; n];

    for i in 0..n - 1 {
        for j in i + 1..n {
            adj[i][j] = d[i][j - i - 1];
            adj[j][i] = d[i][j - i - 1];
        }
    }

    // dp[s]
    // 集合 s の辺を選択したときの最大重み
    let mut dp = vec![0; 1 << n];

    for s in 1_usize..1 << n {
        for i in 0..n - 1 {
            let bi = 1 << i;
            for j in i + 1..n {
                let bj = 1 << j;
                if s & bi > 0 && s & bj > 0 {
                    dp[s] = dp[s].max(dp[s & !(bi | bj)] + adj[i][j]);
                }
            }
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}

#[allow(dead_code)]
fn solve() {
    // 全探索
    input! {
        n: usize,
    }

    let mut d = vec![];

    for i in 0..n - 1 {
        input! {
            di: [usize; n - i - 1],
        }

        d.push(di);
    }

    let mut adj = vec![vec![0; n]; n];

    for i in 0..n - 1 {
        for j in i + 1..n {
            adj[i][j] = d[i][j - i - 1];
            adj[j][i] = d[i][j - i - 1];
        }
    }

    let mut used = vec![false; n];
    let mut ans = 0;

    if n % 2 == 0 {
        ans = dfs(&mut used, &adj, n);
    } else {
        for v in 0..n {
            used[v] = true;
            ans = ans.max(dfs(&mut used, &adj, n));
            used[v] = false;
        }
    }

    println!("{}", ans);
}

fn dfs(used: &mut [bool], adj: &[Vec<usize>], n: usize) -> usize {
    if used.iter().all(|&x| x) {
        return 0;
    }

    let mut ret = 0;

    let first = used.iter().position(|&x| !x).unwrap();
    used[first] = true;

    for second in first + 1..n {
        if !used[second] {
            used[second] = true;
            ret = ret.max(adj[first][second] + dfs(used, adj, n));
            used[second] = false;
        }
    }

    used[first] = false;
    ret
}
