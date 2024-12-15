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
        d: usize, n: usize,
        lrh: [(Usize1, Usize1, usize); n],
    }

    let mut s = vec![24; d];

    for (l, r, h) in lrh {
        for i in l..=r {
            s[i] = s[i].min(h);
        }
    }

    println!("{}", s.iter().sum::<usize>());
}
