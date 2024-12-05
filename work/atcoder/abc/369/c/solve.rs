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
        a: [isize; n],
    }

    // 尺取り法（等差数列になる区間の数）
    let mut right = 0;
    let mut ans = 0;
    let mut d = None;

    for left in 0..n {
        if left == right {
            d = None;
        }

        while right + 1 < n && (d.is_none() || a[right + 1] - a[right] == d.unwrap()) {
            if d.is_none() {
                d = Some(a[right + 1] - a[right]);
            }
            right += 1;
        }

        ans += right - left + 1;
    }

    println!("{}", ans);
}
