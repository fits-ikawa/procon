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
        n: usize, mut m: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort();

    let mut ans = 0;

    for (a, b) in ab {
        if m >= b {
            ans += a * b;
            m -= b;
        } else {
            ans += a * m;
            break;
        }
    }

    println!("{}", ans);
}
