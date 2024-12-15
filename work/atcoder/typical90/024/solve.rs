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
        a: [usize; n],
        b: [usize; n],
    }

    let diff = izip!(a, b).map(|(ai, bi)| ai.abs_diff(bi)).sum::<usize>();

    if diff > k {
        println!("No");
        return;
    }

    println!("{}", if (k - diff) % 2 == 0 { "Yes" } else { "No" });
}
