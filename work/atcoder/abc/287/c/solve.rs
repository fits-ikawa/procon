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
        uv: [(Usize1, Usize1); m],
    }

    if m != n - 1 {
        println!("No");
        return;
    }

    let mut adj = vec![vec![]; n];

    for (u, v) in uv {
        adj[u].push(v);
        adj[v].push(u);
    }

    let start = adj.iter().position(|a| a.len() == 1).unwrap();

    let mut todo = VecDeque::new();
    let mut seen = vec![false; n];
    todo.push_back(start);
    seen[start] = true;

    for _ in 0..n - 1 {
        if let Some(from) = todo.pop_front() {
            for &to in &adj[from] {
                if !seen[to] {
                    seen[to] = true;
                    todo.push_back(to);
                    break; // パスグラフなら行先は 1 個しかないので打ち切る
                }
            }
        }
    }

    println!("{}", if seen.iter().all(|&x| x) { "Yes" } else { "No" });
}
