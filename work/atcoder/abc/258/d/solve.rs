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
        n: usize, x: usize,
        ab: [(usize, usize); n],
    }

    let mut time = 0;
    let mut clear = 0;
    let mut ans = usize::MAX;

    for &(a, b) in &ab[..n.min(x)] {
        time += a + b;
        clear += 1;

        ans = ans.min(time + x.saturating_sub(clear) * b);
    }

    println!("{}", ans);
}
