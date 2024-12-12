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
    // DP で解く
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize
    }

    let mut trap = vec![false; x + 1];
    for bi in b {
        trap[bi] = true;
    }

    // dp[i]
    // i 段目に到達可能かどうか
    let mut dp = vec![false; x + 1];
    dp[0] = true;

    for i in 1..=x {
        if !trap[i] {
            for &ai in &a {
                if i >= ai {
                    dp[i] |= dp[i - ai];
                }
            }
        }
    }

    println!("{}", if dp[x] { "Yes" } else { "No" })
}

#[allow(dead_code)]
fn solve() {
    // BFS で解く
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize
    }

    let mut trap = vec![false; x + 1];
    for bi in b {
        trap[bi] = true;
    }

    let mut todo = VecDeque::new();
    let mut seen = vec![false; x + 1];
    todo.push_back(0);
    seen[0] = true;

    while let Some(from) = todo.pop_front() {
        for &ai in &a {
            let to = from + ai;
            if to <= x && !seen[to] && !trap[to] {
                seen[to] = true;
                todo.push_back(to);
            }
        }
    }

    println!("{}", if seen[x] { "Yes" } else { "No" })
}
