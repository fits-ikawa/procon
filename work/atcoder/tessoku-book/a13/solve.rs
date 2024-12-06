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

    // 尺取り法（組み合わせの合計が k 以下）
    let mut right = 0;
    let mut count = 0;

    for left in 0..n - 1 {
        while right + 1 < n && a[right + 1] - a[left] <= k {
            right += 1;
        }

        // ここでは必ず a[right] - a[left] <= k が満たされている
        // left == right の場合は組み合わせとしてありえないが
        // right - left = 0 になるので無視される
        count += right - left;

        if left == right && right + 1 < n {
            right += 1;
        }
    }

    println!("{}", count);
}
