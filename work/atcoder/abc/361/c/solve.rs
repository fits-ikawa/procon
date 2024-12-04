#![allow(unused_imports)]
use itertools::*;
use itertools_num::structs::Cumsum;
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
        mut a: [usize; n],
    }

    a.sort();

    let mut ans = usize::MAX;

    for w in a.windows(n - k) {
        let diff = w[0].abs_diff(*w.last().unwrap());
        ans = ans.min(diff);
    }

    println!("{}", ans);
}
