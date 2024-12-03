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
        mut a: [usize; n],
    }

    a.sort();

    let sum = a.iter().sum::<usize>() * (n - 1);

    let mut right = n;
    let mut count = 0;

    for left in 0..n - 1 {
        right = right.max(left + 1);
        while right - 1 > left && a[left] + a[right - 1] >= 100000000 {
            right -= 1;
        }
        count += n - right;
    }

    let ans = sum - count * 100000000;

    println!("{}", ans);
}
