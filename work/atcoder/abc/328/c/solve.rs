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
        s: Chars,
        lr: [(Usize1, Usize1); q],
    }

    let mut acc = vec![0; n + 1];

    for i in 0..n - 1 {
        acc[i + 1] = acc[i] + if s[i] == s[i + 1] { 1 } else { 0 };
    }
    acc[n] = acc[n - 1];

    for (l, r) in lr {
        println!("{}", acc[r] - acc[l]);
    }
}
