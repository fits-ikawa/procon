#![allow(unused_imports)]
use itertools::*;
use itertools_num::*;
use maplit::*;
use memoise::memoise_map;
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
    }

    println!("{}", f(n));
}

#[memoise_map(x)]
fn f(x: usize) -> usize {
    if x == 0 {
        return 1;
    }

    f(x / 2) + f(x / 3)
}
