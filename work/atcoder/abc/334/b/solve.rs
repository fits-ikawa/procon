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
        a: i128, m: i128, l: i128, r: i128,
    }

    let l = l - a;
    let r = r - a;

    let leftmost = div_ceil(l, m) * m;

    let ans = if leftmost > r {
        0
    } else {
        (r - leftmost) / m + 1
    };

    println!("{}", ans);
}

fn div_ceil(a: i128, b: i128) -> i128 {
    if (a < 0) == (b < 0) {
        (a + b - 1) / b
    } else {
        a / b
    }
}
