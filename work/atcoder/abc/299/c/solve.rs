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

    if runlen.len() <= 1 {
        println!("-1");
        return;
    }

    let ans = runlen
        .iter()
        .filter_map(|&(len, val)| if val == 'o' { Some(len) } else { None })
        .max()
        .unwrap();

    println!("{}", ans);
}
