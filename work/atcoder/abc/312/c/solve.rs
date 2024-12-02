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
        n: usize, m: usize,
        mut a: [usize; n],
        mut b: [usize; m],
    }

    a.sort();
    b.sort();

    let mut left = 0;
    let mut right = 2000000000;

    while right - left > 1 {
        let mid = (left + right) / 2;

        let seller = a.upper_bound(&mid);
        let buyer = m - b.lower_bound(&mid);

        if seller >= buyer {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", right);
}
