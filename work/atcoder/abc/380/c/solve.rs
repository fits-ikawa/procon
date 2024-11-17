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
        _n: usize, k: usize,
        s: Chars,
    }

    let mut grouped = s
        .into_iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, g)| g.collect_vec())
        .collect_vec();

    let i = if grouped[0][0] == '0' {
        k * 2 - 1
    } else {
        k * 2 - 2
    };

    grouped.swap(i - 1, i);

    println!("{}", grouped.into_iter().flatten().join(""));
}
