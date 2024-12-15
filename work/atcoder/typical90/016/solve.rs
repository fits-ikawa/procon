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
        a: usize, b: usize, c: usize,
    }

    let mut ans = usize::MAX;

    for i in 0..=(n / a).min(9999) {
        for j in 0..=(n / b).min(9999 - i) {
            let m = a * i + b * j;
            if m > n {
                break;
            }
            let k = (n - m) / c;
            if m + c * k == n {
                ans = ans.min(i + j + k);
            }
        }
    }

    println!("{}", ans);
}
