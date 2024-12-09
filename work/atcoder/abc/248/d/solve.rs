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
        a: [usize; n],
        q: usize,
        lrx: [(usize, usize, usize); q],
    }

    let mut idx = vec![vec![]; n + 1];

    for i in 0..n {
        idx[a[i]].push(i + 1);
    }

    for (l, r, x) in lrx {
        let ans = idx[x].upper_bound(&r) - idx[x].lower_bound(&l);
        println!("{}", ans);
    }
}
