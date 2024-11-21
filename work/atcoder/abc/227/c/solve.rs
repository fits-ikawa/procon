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

    let a_max = n.cbrt();
    let b_max = n.sqrt();
    let mut ans = 0;

    // 全検索
    // A の最大値は N の三乗根
    // B の最大値は N の二乗根
    for a in 1..=a_max {
        for b in a..=b_max {
            let c_max = n / (a * b);
            if c_max < b {
                break;
            }
            ans += c_max - b + 1;
        }
    }

    println!("{}", ans);
}

#[allow(dead_code)]
fn solve() {
    input! {
        n: usize,
    }

    let b_max = n.sqrt();
    let mut ans = 0;

    // B は N の二乗根まで動ける
    // B を決めて、それに対して可能な A, C を調べる
    for b in 1..=b_max {
        let mut left = 1;
        let mut right = b;

        if n / (b * b) >= b {
            // A == B まで A を動かせる
            left = right;
        } else {
            // B に対する A の最大値を求める
            while right - left > 1 {
                let mid = (left + right) / 2;

                if n / (mid * b) >= b {
                    left = mid;
                } else {
                    right = mid;
                }
            }
        }

        // A は [1, left] で動けるので、
        // それぞれに対する可能な C の数を列挙
        for a in 1..=left {
            let max_c = n / (a * b);
            ans += max_c - b + 1;
        }
    }

    println!("{}", ans);
}
