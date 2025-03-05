#![allow(clippy::comparison_chain)]
#![allow(clippy::map_entry)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
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

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let sum = x.iter().sum::<usize>();
    let mut ans = usize::MAX;

    for p in [sum / n, sum / n + 1] {
        ans = ans.min(x.iter().map(|&xi| xi.abs_diff(p).pow(2)).sum::<usize>());
    }

    println!("{}", ans);
}

#[fastout]
fn solve() {
    input! {
        n: usize,
        x: [usize; n],
    }

    let mut ans = usize::MAX;

    for p in 1..=100 {
        ans = ans.min(x.iter().map(|&xi| xi.abs_diff(p).pow(2)).sum::<usize>());
    }

    println!("{}", ans);
}
