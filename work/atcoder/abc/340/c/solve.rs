#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use memoise::memoise_map;
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
    }

    println!("{}", dfs(n));
}

#[memoise_map(x)]
fn dfs(x: usize) -> usize {
    if x == 1 {
        return 0;
    }

    x + dfs(x / 2) + dfs((x + 1) / 2)
}
