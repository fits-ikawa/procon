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
        n: usize, m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut adj = vec![vec![]; n];

    for (a, b, c) in abc {
        adj[a].push((b, c));
        adj[b].push((a, c));
    }

    // ダイクストラ法
    let mut todo = BinaryHeap::new();
    let mut cost = vec![usize::MAX; n];
    todo.push((Reverse(0), 0));
    cost[0] = 0;

    while let Some((Reverse(c), from)) = todo.pop() {
        if c > cost[from] {
            continue;
        }

        for &(to, d) in &adj[from] {
            if cost[to] > c + d {
                cost[to] = c + d;
                todo.push((Reverse(c + d), to));
            }
        }
    }

    for i in 0..n {
        if cost[i] == usize::MAX {
            println!("-1");
        } else {
            println!("{}", cost[i]);
        }
    }
}
