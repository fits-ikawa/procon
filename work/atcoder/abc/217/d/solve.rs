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
        l: usize, q: usize,
        cx: [(usize, usize); q],
    }

    let mut cut = btreeset! {0, l};

    for (c, x) in cx {
        if c == 1 {
            cut.insert(x);
        } else {
            let left = cut.range(..x).next_back().copied().unwrap();
            let right = cut.range(x..).next().copied().unwrap();
            println!("{}", right - left);
        }
    }
}
