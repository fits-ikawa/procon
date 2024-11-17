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
        ab: [(usize, usize); n-1],
        px: [(usize, usize); q],
    }

    let mut adj = vec![vec![]; n];
    let mut cnt = vec![0; n];

    for (a, b) in ab {
        adj[a - 1].push(b - 1);
        adj[b - 1].push(a - 1);
    }

    for (p, x) in px {
        cnt[p - 1] += x;
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![false; n];
    todo.push_back(0);
    seen[0] = true;

    while !todo.is_empty() {
        let i = todo.pop_front().unwrap();
        for &j in &adj[i] {
            if !seen[j] {
                cnt[j] += cnt[i];
                seen[j] = true;
                todo.push_back(j);
            }
        }
    }

    println!("{}", cnt.iter().join(" "));
}
