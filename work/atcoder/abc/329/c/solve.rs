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
        _n: usize,
        s: Chars,
    }

    let runlen = s.into_iter().dedup_with_count().collect_vec();

    let mut count = hashmap! {};

    for (v, k) in runlen {
        let value = count.entry(k).or_insert(0);
        *value = (*value).max(v);
    }

    println!("{}", count.values().sum::<usize>());
}
