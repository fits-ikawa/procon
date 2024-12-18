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
        cp: [(usize, usize); n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let acc1 = std::iter::once(0)
        .chain(cp.iter().map(|&(c, p)| if c == 1 { p } else { 0 }))
        .cumsum::<usize>()
        .collect_vec();

    let acc2 = std::iter::once(0)
        .chain(cp.iter().map(|&(c, p)| if c == 2 { p } else { 0 }))
        .cumsum::<usize>()
        .collect_vec();

    for (l, r) in lr {
        println!("{} {}", acc1[r] - acc1[l - 1], acc2[r] - acc2[l - 1]);
    }
}
