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
        h: usize, w: usize,
        mut a: [[isize; w]; h],
        b: [[isize; w]; h],
    }

    let mut ans = 0;

    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let diff = b[i][j] - a[i][j];
            a[i][j] += diff;
            a[i + 1][j] += diff;
            a[i][j + 1] += diff;
            a[i + 1][j + 1] += diff;
            ans += diff.abs();
        }
    }

    if a == b {
        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
    }
}
