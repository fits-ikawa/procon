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
        n: usize, k: usize,
        r: [usize; n],
    }

    let a = r.iter().map(|&ri| (1..=ri).collect_vec()).collect_vec();
    let mut ans = vec![];

    for seq in a.into_iter().multi_cartesian_product() {
        if seq.iter().sum::<usize>() % k == 0 {
            ans.push(seq);
        }
    }

    ans.sort();

    if !ans.is_empty() {
        println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
    }
}
