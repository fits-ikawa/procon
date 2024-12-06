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
        mut a: [usize; n],
    }

    a.sort();

    // 尺取り法（長さ m の最も多く含む要素を含む区間の、要素数）
    let mut ans = 0;

    let mut right = 0;
    let mut p = 1;

    for left in 0..n {
        while right + 1 < n && a[right + 1] - a[left] < m {
            right += 1;
            p += 1;
        }

        ans = ans.max(p);
        p -= 1;

        if left == right && right + 1 < n {
            right += 1;
        }
    }

    println!("{}", ans);
}
