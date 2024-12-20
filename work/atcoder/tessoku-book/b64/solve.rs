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
    let mut prev = vec![0; n]; // prev[0] は不定

    todo.push((Reverse(0), 0));

    while let Some((Reverse(c), from)) = todo.pop() {
        if c > cost[from] {
            continue;
        }

        for &(to, d) in &adj[from] {
            if cost[to] > c + d {
                cost[to] = c + d;
                prev[to] = from;
                todo.push((Reverse(c + d), to));
            }
        }
    }

    let mut cur = n - 1;
    let mut path = vec![];
    while cur != 0 {
        path.push(cur + 1);
        cur = prev[cur];
    }
    path.push(1);

    println!("{}", path.into_iter().rev().join(" "));
}
