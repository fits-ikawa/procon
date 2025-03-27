#![allow(clippy::comparison_chain)]
#![allow(clippy::collapsible_else_if)]
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
    // 解説 AC
    input! {
        n: usize, m: usize,
        h: [isize; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        let (mut u, mut v) = (u, v);
        if h[u] < h[v] {
            std::mem::swap(&mut u, &mut v);
        }

        adj[u].push((v, 0));
        adj[v].push((u, h[u] - h[v]));
    }

    // ダイクストラ法
    let mut todo = BinaryHeap::new();
    let mut cost = vec![isize::MAX; n];

    todo.push((Reverse(0), 0));
    cost[0] = 0;

    while let Some((Reverse(c), from)) = todo.pop() {
        if c > cost[from] {
            continue;
        }

        for &(to, d) in &adj[from] {
            if c + d < cost[to] {
                cost[to] = c + d;
                todo.push((Reverse(c + d), to));
            }
        }
    }

    let ans = (0..n).map(|i| h[0] - h[i] - cost[i]).max().unwrap();

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    // 嘘解法？
    input! {
        n: usize, m: usize,
        h: [isize; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut map = btreemap! {};

    for i in 0..n {
        let value = map.entry(h[i]).or_insert(vec![]);
        value.push(i);
    }

    let mut score = vec![0; n];

    for (&_, vs) in &map {
        for &v in vs {
            if score[v] == 0 {
                let mut todo = BinaryHeap::new();
                todo.push((0, v));

                while let Some((s, from)) = todo.pop() {
                    if score[from] > s {
                        continue;
                    }

                    for &to in &adj[from] {
                        let new_score = if h[from] == h[to] {
                            s
                        } else if h[from] < h[to] {
                            s + h[to] - h[from]
                        } else {
                            s - (h[from] - h[to]) * 2
                        };

                        if new_score > score[to] {
                            score[to] = new_score;
                            todo.push((new_score, to));
                        }
                    }
                }
            }
        }
    }

    println!("{}", score[0]);
}
