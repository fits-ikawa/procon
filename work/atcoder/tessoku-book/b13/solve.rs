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
    }

    // 尺取り法（和が k 以下となるような区間の数）
    let mut right = 0;
    let mut sum = a[0];
    let mut count = 0;

    for left in 0..n {
        while right + 1 < n && sum + a[right + 1] <= k {
            right += 1;
            sum += a[right];
        }

        // 長さ 1 の列で K より大きいケースがあるのでチェックが必要
        if sum <= k {
            count += right - left + 1;
        }
        sum -= a[left];

        if left == right && right + 1 < n {
            right += 1;
            sum += a[right];
        }
    }

    println!("{}", count);
}
