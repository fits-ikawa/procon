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
        k: [usize; n],
    }

    let sum = k.iter().sum::<usize>();
    let mut ans = usize::MAX;

    // ビット反転のパターンは同じ結果なので 2^n / 2 まで調べればいい
    for i in 0..(1 << n) / 2 {
        let mut a = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                a += k[j];
            }
        }
        ans = ans.min(a.max(sum - a));
    }

    println!("{}", ans);
}
