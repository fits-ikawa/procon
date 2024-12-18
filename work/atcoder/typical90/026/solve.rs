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
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![None; n]; // 偶奇を記録
    let mut even = vec![];
    let mut odd = vec![];
    todo.push_back(0);
    seen[0] = Some(false);

    while let Some(from) = todo.pop_front() {
        if seen[from].unwrap() {
            odd.push(from)
        } else {
            even.push(from)
        }

        for &to in &adj[from] {
            if seen[to].is_none() {
                seen[to] = seen[from].map(|x| !x);
                todo.push_back(to);
            }
        }
    }

    let ans = if even.len() > odd.len() { &even } else { &odd }
        .iter()
        .take(n / 2)
        .map(|x| x + 1)
        .collect_vec();

    println!("{}", ans.iter().join(" "));
}
