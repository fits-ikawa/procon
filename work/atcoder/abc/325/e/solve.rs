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
        n: usize, a: usize, b: usize, c: usize,
        d: [[usize; n]; n],
    }

    // adj[k][i][j]
    // k (0: 社用車, 1: 電車) を使って都市 i から j へ移動するときにかかる時間
    let mut adj = vec![vec![vec![0; n]; n]; 2];

    for i in 0..n {
        for j in 0..n {
            adj[0][i][j] = d[i][j] * a;
            adj[1][i][j] = d[i][j] * b + c;
        }
    }

    // ダイクストラ法
    let mut todo = BinaryHeap::new();
    let mut cost = vec![vec![usize::MAX; n]; 2];

    todo.push((Reverse(0), (0, 0)));
    cost[0][0] = 0;

    while let Some((Reverse(c), (k, from))) = todo.pop() {
        if c > cost[k][from] {
            continue;
        }

        if k == 0 && c < cost[1][from] {
            cost[1][from] = c;
            todo.push((Reverse(c), (1, from)));
        }

        for to in 0..n {
            let d = c + adj[k][from][to];
            if cost[k][to] > d {
                cost[k][to] = d;
                todo.push((Reverse(d), (k, to)));
            }
        }
    }

    // 目的地には必ず辿り着けるので単純にコストを比較
    println!("{}", cost[0][n - 1].min(cost[1][n - 1]));
}
