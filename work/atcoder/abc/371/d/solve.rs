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
        x: [isize; n],
        p: [usize; n],
        q: usize,
        lr: [(isize, isize); q],
    }

    let acc = std::iter::once(0).chain(p).cumsum::<usize>().collect_vec();

    for (l, r) in lr {
        let lpos = x.upper_bound(&(l - 1));
        let rpos = x.upper_bound(&r);
        println!("{}", acc[rpos] - acc[lpos]);
    }
}
