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
        n: usize, k: usize,
        mut ab: [(usize, usize); n],
    }

    let mut sum = ab.iter().map(|&(_, b)| b).sum::<usize>();

    if sum <= k {
        println!("1");
        return;
    }

    ab.sort();

    for (a, b) in ab {
        sum -= b;
        if sum <= k {
            println!("{}", a + 1);
            return;
        }
    }
}
