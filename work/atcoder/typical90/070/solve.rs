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
        xy: [(isize, isize); n],
    }

    let mut xs = vec![];
    let mut ys = vec![];

    for (x, y) in xy {
        xs.push(x);
        ys.push(y);
    }

    xs.sort();
    ys.sort();

    let x_median = xs[xs.len() / 2];
    let y_median = ys[ys.len() / 2];

    let ans = xs.iter().map(|&x| x.abs_diff(x_median)).sum::<usize>()
        + ys.iter().map(|&y| y.abs_diff(y_median)).sum::<usize>();

    println!("{}", ans);
}
