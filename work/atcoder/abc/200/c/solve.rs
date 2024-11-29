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
        a: [isize; n],
    }

    let mut b = vec![0_usize; 200];

    for ai in a {
        b[ai as usize % 200] += 1;
    }

    let ans = b
        .iter()
        .map(|&bi| bi * bi.saturating_sub(1) / 2)
        .sum::<usize>();

    println!("{}", ans);
}
