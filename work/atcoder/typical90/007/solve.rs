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
        mut a: [usize; n],
        q: usize,
        b: [usize; q],
    }

    a.sort();

    for bi in b {
        let pos = a.upper_bound(&bi);

        let ans = if pos == 0 {
            bi.abs_diff(a[0])
        } else if pos == n {
            bi.abs_diff(a[n - 1])
        } else {
            bi.abs_diff(a[pos]).min(bi.abs_diff(a[pos - 1]))
        };

        println!("{}", ans);
    }
}
