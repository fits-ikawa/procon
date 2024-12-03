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
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }

    let mut sums = hashset! {};

    for (i, j, k) in iproduct!(0..n, 0..m, 0..l) {
        sums.insert(a[i] + b[j] + c[k]);
    }

    for xi in x {
        println!("{}", if sums.contains(&xi) { "Yes" } else { "No" });
    }
}
