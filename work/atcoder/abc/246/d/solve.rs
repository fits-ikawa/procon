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

    let mut a = 0;
    let mut max_right = 1000000;

    let mut ans = usize::MAX;

    // a を 1 ずつ増やしながら、対応する最小の b を二分探索で見つける
    while a <= max_right {
        let mut left = a;
        let mut right = max_right;

        if f(a, a) >= n {
            // 検索範囲がすべて（＝最小値ですでに）条件を満たすケース
            right = a;
        } else {
            while right - left > 1 {
                let mid = (left + right) / 2;
                if f(a, mid) >= n {
                    right = mid;
                } else {
                    left = mid;
                }
            }
        }

        ans = ans.min(f(a, right));
        max_right = right;
        a += 1;
    }

    println!("{}", ans);
}

fn f(a: usize, b: usize) -> usize {
    a * a * a + a * a * b + a * b * b + b * b * b
}
