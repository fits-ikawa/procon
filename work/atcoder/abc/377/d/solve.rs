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
        n: usize, m: usize,
        lr: [(usize, usize); n],
    }

    let mut nearest = vec![0; m + 1];

    for &(l, r) in &lr {
        nearest[r] = nearest[r].max(l);
    }

    // 尺取り法（区間 lr を一つも完全に含まない区間の数）
    let mut right = 1;
    let mut ans = 0;

    for left in 1..=m {
        while nearest[right] < left && right < m && nearest[right + 1] < left {
            right += 1;
        }

        if nearest[right] < left {
            ans += right - left + 1;
        }

        if left == right && right < m {
            right += 1;
        }
    }

    println!("{}", ans);
}

#[allow(clippy::needless_range_loop)]
#[allow(dead_code)]
fn solve() {
    // 右を固定する尺取り法（解説 AC）
    input! {
        n: usize, m: usize,
        lr: [(usize, usize); n],
    }

    let mut nearest = vec![0; m + 1];

    for &(l, r) in &lr {
        nearest[r] = nearest[r].max(l);
    }

    let mut left = 0;
    let mut ans = 0;

    for right in 1..=m {
        left = left.max(nearest[right]);
        ans += right - left;
    }

    println!("{}", ans);
}
