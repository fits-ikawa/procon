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
        mut a: [usize; n],
        mut b: [usize; m],
    }

    b.sort();

    let mut ans = usize::MAX;

    for ai in a {
        ans = ans.min(match b.lower_bound(&ai) {
            0 => ai.abs_diff(b[0]),
            p if p >= m => ai.abs_diff(b[m - 1]),
            other => ai.abs_diff(b[other]).min(ai.abs_diff(b[other - 1])),
        });
    }

    println!("{}", ans);
}
