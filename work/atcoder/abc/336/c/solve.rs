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
    }

    // 0 が 1 番目なので n - 1 の 5 進数を考える
    let mut n = n - 1;
    let mut digits = vec![];

    while n > 0 {
        digits.push(n % 5);
        n /= 5;
    }

    let ans = digits.iter().rev().map(|d| d * 2).join("");

    println!("{}", ans);
}
