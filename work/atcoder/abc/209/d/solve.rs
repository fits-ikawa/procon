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
        n: usize, q: usize,
        ab: [(Usize1, Usize1); n-1],
        cd: [(Usize1, Usize1); q],
    }

    let mut adj = vec![vec![]; n];

    for (a, b) in ab {
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![None; n]; // 偶奇を記録
    todo.push_back(0);
    seen[0] = Some(false);

    while let Some(from) = todo.pop_front() {
        for &to in &adj[from] {
            if seen[to].is_none() {
                seen[to] = Some(!seen[from].unwrap());
                todo.push_back(to);
            }
        }
    }

    for (c, d) in cd {
        println!(
            "{}",
            if seen[c].unwrap() == seen[d].unwrap() {
                "Town"
            } else {
                "Road"
            }
        );
    }
}
