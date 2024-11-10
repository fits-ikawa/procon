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
        lr: [(usize, usize); n],
    }

    let mut nearest = vec![0; m + 1];

    for &(l, r) in &lr {
        nearest[r] = nearest[r].max(l);
    }

    let mut left = 0;
    let mut ans = 0;

    for right in 1..=m {
        left = left.max(nearest[right]);
        ans += right - left;
    }

    println!("{}", ans);
}
