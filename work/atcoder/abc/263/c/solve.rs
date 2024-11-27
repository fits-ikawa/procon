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
    }

    for c in (1..=m).combinations(n) {
        println!("{}", c.iter().join(" "));
    }
}

#[allow(dead_code)]
fn dfs(a: &mut Vec<usize>, n: usize, m: usize) {
    if a.len() == n {
        println!("{}", a.iter().join(" "));
        return;
    }

    let prev = if a.is_empty() { &0 } else { a.last().unwrap() };

    for i in prev + 1..=m {
        a.push(i);
        dfs(a, n, m);
        a.pop();
    }
}
