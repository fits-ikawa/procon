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
        n: usize, mut k: isize, x: isize,
        mut a: [isize; n],
    }

    // 0 円未満にするとき有利な価格の順にソート
    a.sort_by_key(|ai| x - ai % x);

    for i in 0..=1 {
        // i: 0 -> 0 円未満にしない範囲でクーポンを使用
        // i: 1 -> 0 円未満を許してクーポンを使用
        for j in 0..n {
            let k_use = k.min(a[j] / x + i);
            a[j] = (a[j] - x * k_use).max(0);
            k -= k_use;
        }
    }

    println!("{}", a.iter().sum::<isize>());
}
