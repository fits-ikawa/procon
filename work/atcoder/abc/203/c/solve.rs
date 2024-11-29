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
        n: usize, mut k: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort();

    let mut cur = 0;

    for (a, b) in ab {
        if a - cur > k {
            break;
        }

        k -= a - cur;
        k += b;
        cur = a;
    }

    cur += k;

    println!("{}", cur);
}
