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
        n: usize, mut w: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort();
    ab.reverse();

    let mut ans = 0;

    for (a, b) in ab {
        if w >= b {
            ans += a * b;
            w -= b;
        } else {
            ans += a * w;
            break;
        }
    }

    println!("{}", ans);
}
