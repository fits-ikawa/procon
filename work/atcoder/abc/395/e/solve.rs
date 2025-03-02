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
        n: usize, m: usize, x: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![vec![]; 2]; n];

    for (u, v) in uv {
        adj[u][0].push(v);
        adj[v][1].push(u);
    }

    // ダイクストラ法
    let mut todo = BinaryHeap::new();
    let mut cost = vec![[usize::MAX; 2]; n];
    todo.push((Reverse(0), 0, 0));
    cost[0][0] = 0;

    while let Some((Reverse(c), from, world)) = todo.pop() {
        if c > cost[from][world] {
            continue;
        }

        let another = (world + 1) % 2;
        if cost[from][another] > c + x {
            cost[from][another] = c + x;
            todo.push((Reverse(c + x), from, another));
        }

        for &to in &adj[from][world] {
            if cost[to][world] > c + 1 {
                cost[to][world] = c + 1;
                todo.push((Reverse(c + 1), to, world));
            }
        }
    }

    println!("{}", cost[n - 1][0].min(cost[n - 1][1]));
}
