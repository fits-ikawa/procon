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
        ab: [(isize, isize); n],
    }

    let ans = ab
        .iter()
        .combinations(3)
        .filter(|c| {
            // 外積で判定
            let &(x1, y1) = c[0];
            let &(x2, y2) = c[1];
            let &(x3, y3) = c[2];
            (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1) != 0
        })
        .count();

    println!("{}", ans);
}
