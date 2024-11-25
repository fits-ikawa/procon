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
        p: [[usize; 3]; n],
    }

    let scores = p.iter().map(|x| x.iter().sum::<usize>()).collect_vec();

    let mut scores_dup = scores.clone();
    let (_, &mut border, _) = scores_dup.select_nth_unstable_by_key(k - 1, |&s| Reverse(s));

    for s in scores {
        println!("{}", if s + 300 >= border { "Yes" } else { "No" });
    }
}
