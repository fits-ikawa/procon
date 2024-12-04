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
        n: usize, x: usize, y:usize,
        a: [usize; n],
        b: [usize; n],
    }

    let acc_a = a.iter().sorted().rev().cumsum::<usize>().collect_vec();
    let acc_b = b.iter().sorted().rev().cumsum::<usize>().collect_vec();

    let max_sweet = (acc_a.upper_bound(&x) + 1).min(n);
    let max_salty = (acc_b.upper_bound(&y) + 1).min(n);

    println!("{}", max_sweet.min(max_salty));
}
