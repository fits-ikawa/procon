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
        s: Chars,
    }

    let mut h1 = vec![1; n];
    let mut h2 = vec![1; n];

    for i in 0..n - 1 {
        if s[i] == 'A' {
            h1[i + 1] = h1[i] + 1;
        }
    }

    for i in (1..n).rev() {
        if s[i - 1] == 'B' {
            h2[i - 1] = h2[i] + 1;
        }
    }

    let ans = izip!(h1, h2).map(|(a, b)| a.max(b)).sum::<usize>();

    println!("{}", ans);
}
