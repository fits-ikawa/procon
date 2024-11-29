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
        n: usize, t: usize,
        a: [usize; n],
    }

    let acc = a.iter().cumsum::<usize>().collect_vec();
    let sum = a.iter().sum::<usize>();

    let t = t % sum;

    let i = acc.upper_bound(&t);

    println!("{} {}", i + 1, t - if i == 0 { 0 } else { acc[i - 1] });
}
