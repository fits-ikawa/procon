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
        p: [usize; n],
    }

    let mut q = vec![0; n];

    for i in 0..n {
        q[p[i] - 1] = i + 1;
    }

    println!("{}", q.iter().join(" "));
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let ans = p
        .into_iter()
        .enumerate()
        .map(|(i, pi)| (pi, i + 1))
        .sorted()
        .map(|(_, i)| i)
        .collect_vec();

    println!("{}", ans.iter().join(" "));
}
